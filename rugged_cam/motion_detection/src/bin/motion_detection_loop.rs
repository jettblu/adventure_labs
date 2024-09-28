use tokio::sync::mpsc;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use motion_detection::{ MotionDetector, ParamsMotionDetector, MsgMotionDetector };
use log::{ info, trace, warn, LevelFilter };

#[tokio::main]
async fn main() {
    let _ = simple_logging::log_to_file("logs/motion_detection/test.log", LevelFilter::Info);
    let token = CancellationToken::new();
    let child_token = token.child_token();
    let (tx, mut rx) = mpsc::channel(1);
    let params = ParamsMotionDetector {
        latency_millis: 200,
        pin_number: 24,
        sender: tx.clone(),
        token_cancellation: child_token,
    };
    let motion_detector = MotionDetector::new(params);
    tokio::spawn(async move {
        motion_detector.run().await;
    });
    // safety hard cap on run time
    let duration_cap_secs: u64 = 60 * 120;
    let start_time: SystemTime = SystemTime::now();
    loop {
        let val: Option<MsgMotionDetector> = rx.recv().await;
        if val.is_none() {
            println!("No value received");
        } else {
            let msg = val.unwrap();
            let time_since_start = msg.time
                .duration_since(start_time.clone())
                .expect("Invalid time operation")
                .as_secs();
            if time_since_start > duration_cap_secs {
                break;
            }
            // check detection result
            if msg.motion_detected {
                // log detection to file
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                info!("{} {}", now, "motion detected");
            } else {
                // pass for now
            }
        }
    }
}
