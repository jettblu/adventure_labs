use std::error::Error;
use tokio::time;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::time::SystemTime;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

pub struct MotionDetector {
    sender: Sender<MsgMotionDetector>,
    token_cancellation: CancellationToken,
    pin_number: u8,
    latency_millis: u64,
}

pub struct ParamsMotionDetector {
    pub sender: Sender<MsgMotionDetector>,
    pub token_cancellation: CancellationToken,
    pub pin_number: u8,
    pub latency_millis: u64,
}

pub struct MsgMotionDetector {
    pub time: SystemTime,
    pub motion_detected: bool,
}

pub enum StatusMotionDetector {
    Running,
    Stopped,
}

impl MotionDetector {
    pub fn new(params: ParamsMotionDetector) -> MotionDetector {
        MotionDetector {
            pin_number: params.pin_number,
            latency_millis: params.latency_millis,
            sender: params.sender,
            token_cancellation: params.token_cancellation,
        }
    }
    pub async fn run(&self) {
        let pin = Gpio::new()
            .expect("unable to create new gpio")
            .get(self.pin_number)
            .expect("unable to get requested pin")
            .into_input();
        loop {
            if self.token_cancellation.is_cancelled() {
                // TODO:M add unique id/stringf name for sensor
                println!("Motion detection cancelled");
                break;
            }
            let is_high = pin.is_high();
            let new_msg: MsgMotionDetector = MsgMotionDetector {
                motion_detected: is_high,
                time: SystemTime::now(),
            };
            let result_send = self.sender.send(new_msg).await;
            if let Err(_) = result_send {
                println!("receiver dropped");
            }
            time::sleep(time::Duration::from_millis(self.latency_millis)).await;
        }
    }
}
