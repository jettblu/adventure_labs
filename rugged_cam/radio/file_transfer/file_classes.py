import os
import time
import file_transfer.utils.packaging_data
from meshtastic import portnums_pb2
import tqdm
import logging

class FileTransferReceiver:
    """Class used to store and handle incoming file packets and should be created when the first request packet is
    acknowledged. Added to as the packets come in. Checked when the last packet number arrives or when a timeout is
    reached"""
    def __init__(self, file_name, file_id, num_packets, interface, sending_id, timeout=30, disable_bar=False):
        self.name = file_name
        self.id = file_id
        self.num_packets = int(num_packets)
        self.interface = interface
        self.sending_id = sending_id
        self.packet_dict = {}
        self.timeout = timeout + 10
        self.last_packet = time.time()
        self.progress_bar = tqdm.tqdm(total=num_packets, unit='packet', disable=disable_bar)
        initial_ack = file_transfer.utils.packaging_data.make_status_packet(file_id, 1)  # Make ack array
        self.send_data(bytes(initial_ack))
        self.kill = False
        self.finished = False
        self.saved = False
        self.logger = logging.getLogger("receiverApp")

    def update(self):
        # after timeout... we either have all of our packets (success) or we don't (failure)
        if time.time()-self.last_packet > self.timeout:
            if self.get_missing_nums():  # Try to see if all the packets somehow made it
                self.logger.log(f'File Transfer "{self.name}" Failed')
                self.progress_bar.close()
                self.kill = True
            else:
                self.logger.log(f'File Transfer "{self.name}" Failed')
                self.save_to_file()
                self.kill = True
                self.progress_bar.close()

    def add_packet(self, packet: bytearray):
        """Send byte array received from radio and it will parse it and save it to the packet_dict
        -Returns true if it added it and false if it didn't"""
        self.last_packet = time.time()
        packet.pop(0)  # gives file id
        num = packet.pop(0)  # Packet index
        if num < self.num_packets:
            self.packet_dict[num] = packet
            self.progress_bar.update(1)
            if len(self.packet_dict) == self.num_packets:
                self.save_to_file()
            return True
        return False

    def manage_com_packet(self, packet: bytearray):
        """Receives and Responds to fcom Packets"""
        self.last_packet = time.time()
        packet_type = packet[5]
        if packet_type == 2:  # Done Transmitting
            missing_packets = self.get_missing_nums()
            if missing_packets:  # Ask for missing packets
                # print(missing_packets)
                now = int(time.time())
                self.logger.info(f'{now} requesting {len(missing_packets)} missing packets')
                ret_packet = file_transfer.utils.packaging_data.make_status_packet(self.id, 3, opt_data=missing_packets)
            else:  # Let it know all packets are received
                now = int(time.time())
                self.logger.info(f'{now} sending confirmation that all packets have been received')
                ret_packet = file_transfer.utils.packaging_data.make_status_packet(self.id, 4)
                self.save_to_file()
                self.kill = True
            self.send_data(ret_packet)

    def get_missing_nums(self):
        """Returns a list of missing packets based on the indexes stored in the packet_dict"""
        missing_nums = []
        for num in range(self.num_packets):
            if num not in self.packet_dict.keys():
                missing_nums.append(num)
        return missing_nums

    def save_to_file(self):

        check = self.get_missing_nums()
        if len(check) == 0 and not self.saved:
            self.progress_bar.close()
            # get file_name from self.name
            file_name = self.name.split('/')[-1]
            # now save to unloading dock with incoming name
            save_destination_path = f'receiving_dock/{file_name}'
            now = int(time.time())
            self.logger.info(f'{now} saving file with incoming name {self.name} to {save_destination_path}')
            with open(save_destination_path, 'wb') as fi:
                for num in range(self.num_packets):
                    fi.write(self.packet_dict[num])
            self.finished = True
            self.saved = self.finished
            return True
        return False

    def send_data(self, data):
        """Sends data over interface to destination"""
        self.interface.sendData(bytes(data), destinationId=self.sending_id, portNum=portnums_pb2.IP_TUNNEL_APP,
                                wantAck=False)


class FileTransferSender:
    """Class used to communicate and send files with a desired target. Sends Initial Packet, receives ack packets and
    sends packets and a confirmation packet at the end."""
    def __init__(self, file_name, file_id: int, interface, destination_id, send_delay=10, packet_len=232,
                 disable_bar=True):
        self.name = file_name
        self.id = file_id  # 0 Reserved for file meta packets
        self.interface = interface
        self.destination_id = destination_id
        self.packet_queue = []
        self.packet_len = packet_len
        data_dict = file_transfer.utils.packaging_data.split_data(file_name, packet_len)  # Unlabeled
        self.data_dict = file_transfer.utils.packaging_data.package_data(data_dict, self.id) # Labeled
        self.delay = send_delay
        self.packet_num = len(self.data_dict)
        self.last_send = time.time()
        # Modes- 0: Send Initial Packet, 2:Send Data, 3:Send Finish
        self.mode = 0
        self.kill = False
        self.finished = False
        # Send initial Req packet
        self.send_initial()
        self.disable_bar = disable_bar
        self.progress_bar = tqdm.tqdm(total=len(data_dict)+1, unit='packet', disable=self.disable_bar)
        self.progress_bar.update()
        self.logger = logging.getLogger("unloaderApp")

    def send_initial(self):
        # Sends initial packet
        init_str = file_transfer.utils.packaging_data.make_initial_req(self.name, len(self.data_dict), self.id)
        self.interface.sendText(init_str, destinationId=self.destination_id)

    def update(self):
        """-Sees if it has a packet to send and can send a packet and sends one if it can
        - Deletes itself if it's been too long and not heard anything"""
        if self.packet_queue and time.time()-self.last_send > self.delay:
            self.last_send = time.time()
            data = self.packet_queue.pop(0)
            # print(f'sending: {data}')
            #TODO: request acknowledgement??
            self.interface.sendData(bytes(data), portNum=portnums_pb2.IP_TUNNEL_APP, destinationId=self.destination_id,
                                    wantAck=False)
            self.progress_bar.update()

        elif time.time()-self.last_send >= self.delay*self.packet_num:
            # alive for too long with no response... end sender process
            now = int(time.time())
            self.logger.error(f'{now} failed Send Timeout')
            self.logger.info(f'{now} time elapsed: time.time() - self.last_send')

            self.kill = True

    def manage_com_packet(self, packet: bytearray):
        """Returns a list of missing packets based on the indexes stored in the packet_dict
        packet_types = {0: Deny initial Request, 1: Confirm Initial Request, 2: Done Transmitting,
        3: Need Packets(list Packets after one byte at a time), 4: Received all Packets(finished)}
        inputs:
            - packet = bytes(0, c, o, m, file_num, packet_type, opt data...)

        Mainly Adds to the sending queue in accordance with the received packet or deletes the object
        """
        packet_type = packet[5]
        if packet_type == 0:
            now = int(time.time())
            self.logger.error(f'{now} Sending denied')
            self.kill = True
        elif packet_type == 1:  # confirm initial requets
            for data in self.data_dict.values():
                self.packet_queue.append(data)
            self.packet_queue.append(file_transfer.utils.packaging_data.make_status_packet(self.id, 2))
        elif packet_type == 3:  # get needed packets
            needed_packets = list(packet[6:])
            for num in needed_packets:
                self.packet_queue.append(self.data_dict[num])
            self.packet_queue.append(file_transfer.utils.packaging_data.make_status_packet(self.id, 2))
            self.progress_bar.close()
            self.progress_bar = tqdm.tqdm(total=len(self.packet_queue), unit='packet', disable=self.disable_bar)
        else:
            # status num 2... transfer complete
            self.progress_bar.close()
            now = int(time.time())
            self.logger.info(f'{now} Confirmed File Transfer #{self.id} Complete')
            self.finished = True
            self.kill = True
        self.last_send = time.time()
