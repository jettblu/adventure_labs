"""Land_and_air
Connect 2 Radios to the USB ports running meshtastic and then run the file. A progress bar should show up
100%|██████████| 36/36 [02:52<00:00,  4.79s/packet] (ex. progress bar) - Long Fast(4 sec send delay) 50 bytes/sec
100%|██████████| 36/36 [01:06<00:00,  1.84s/packet] - Medium Fast(1.6 sec sending delay) 138 bytes/sec
100%|██████████| 36/36 [00:15<00:00,  2.32packet/s] - Short Fast(.4 sec sending delay) 300-500 bytes/sec
"""
import os
import sys

from data_types import ResultSender

from serial.serialutil import SerialException
import time
from meshtastic.serial_interface import SerialInterface
from meshtastic.util import findPorts
from pubsub import pub
import argparse
from file_transfer.file_class_manager import FileTransManager
import logging
Text_Queue = []
Queue = []


def main(interface, time_delay=3, use_dir=False, auto_restart=False, path='', shortname_destination_radio='', max_send_time_secs=60*5):
    logger = logging.getLogger("unloaderApp")
    # Args to be used
    time_delay = int(time_delay)
    size = 0
    paths = []
    if use_dir:
        list_paths = os.listdir(path)
        for i, fname in enumerate(list_paths):
            path = path + '/' + fname
            if not fname.startswith('.') and os.path.isfile(path):
                size += os.path.getsize(path)
                paths.append(path)

    else:
        paths = [path]
        size += os.path.getsize(path)
    try:
        paths = sorted(paths)
        send_time = time_delay * size/232 * 1.1
        send_hrs = int(send_time // 60**2)
        send_mins = int(send_time - send_hrs*60**2)//60
        send_secs = round(send_time - send_hrs*60**2 - send_mins*60)
        now = time.time()
        logger.info(f'{now} Transfer will take approx. {send_hrs}hrs {send_mins}mins {send_secs}s')
        if send_time > max_send_time_secs:
            now = time.time()
            logger.error(f'{now} Estimated transfer time exceeds max send time, please increase the send delay or decrease the file size')
            sys.exit('Error: Estimated transfer time exceeds max send time, please increase the send delay or decrease the file size')
        manager = FileTransManager(interface, send_delay=time_delay, auto_restart=auto_restart)  # Sender
        # Selecting the destination(to be changed)
        nodes = interface.nodes
        nodes.pop(interface.getMyNodeInfo()['user']['id'])
        keys = list(nodes.keys())
        # for i, key in enumerate(keys):
        #     print(f"{i+1}: {key} - {nodes[key]['user']['shortName']}")
        # index = input('>>')
        # get index by shortname
        for i, key in enumerate(keys):
            if nodes[key]['user']['shortName'] == shortname_destination_radio:
                index = i + 1
                break
        selected = keys[int(index)-1]
        destination_id = selected
        # destination_id = interface_2.getMyNodeInfo()['user']['id']
        now = time.time()
        logger.info(f"{now} Starting transfer of {path} to {nodes[selected]['user']['shortName']}")
        manager.send_new_files(paths, destination_id)
        looping = True
        while looping:
            time.sleep(.05)  # For performance lol
            # Sending update
            manager.update_all()
            if Queue:  # Handle binary data
                name, packet = Queue.pop(0)
                payload = packet['decoded']['payload']
                manager.new_data_packet(bytearray(payload))
            if Text_Queue:  # handle text data
                name, packet = Text_Queue.pop(0)
                text = packet['decoded']['text']

            if len(manager.transfer_objects) == 0:
                looping = False
        interface.close()
        return ResultSender.SUCCESS
    except KeyboardInterrupt:
        return ResultSender.FAIL



def on_receive(packet, interface): # called when a packet arrives
    # print(packet['decoded']['portnum'], interface.getShortName())
    # print(f'received_1: {packet["decoded"]["payload"]}')
    if packet['decoded']['portnum'] == 'IP_TUNNEL_APP':
        Queue.append((interface.getShortName(), packet))
    elif packet['decoded']['portnum'] == 'TEXT_MESSAGE_APP':
        Text_Queue.append((interface.getShortName(), packet))


def run_sender(interface, time_delay=3, use_dir=False, auto_restart=False, path='',  shortname_destination_radio='', max_send_time_secs=60*5):
    pub.subscribe(on_receive, "meshtastic.receive")
    result = main(interface, time_delay, use_dir, auto_restart, path, shortname_destination_radio, max_send_time_secs)
    return result

if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        prog='Meshtastic File Sender',
        description='Sends a file or directory to another node running the receiver program',)
    parser.add_argument('-t', '--time_delay', default=3, help='Time between sending packets')
    parser.add_argument('-d', '--use_dir', default=False, help='Sends a directory of files instead')
    parser.add_argument('-r', '--auto_restart', default=False, help='Automatically restart transfer upon '
                                                                    'failure(good for large transfers)')
    parser.add_argument('-p', '--path', required=True, help='Path to find the file from(must be less than'
                                                            '59kb')
    parser.add_argument('-m', '--max_send_tim_secs', default=60*5, help='Max time to send the file in seconds')
    parser.add_argument('-s', '--shortname_destination_radio', required=True, help='Shortname of the destination radio')
    ports = findPorts(True)
    if len(ports) > 1:
        print('Multiple active serial ports found, connecting to the first valid radio...')
    interface = None
    if ports:
        for port in ports:
            try:
                interface = SerialInterface(devPath=port)
                print(f'connected to {interface.getShortName()}')
                break
            except (BlockingIOError, SerialException) as e:
                pass
    if interface:
        pub.subscribe(on_receive, "meshtastic.receive")
        # get args from parser
        args = parser.parse_args()
        main(interface, args.time_delay, args.use_dir, args.auto_restart, args.path, args.shortname_destination_radio, args.max_send_tim_secs)
    else:
        sys.exit('Error: No Radio Available')
