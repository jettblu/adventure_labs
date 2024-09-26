use tokio::sync::mpsc;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use motion_detection::{ MotionDetector, ParamsMotionDetector, MsgMotionDetector };

#[tokio::main]
async fn main() {
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
    // safety hard cap on iterations
    let max_iterations: u32 = 50;
    let duration_cap_secs: u64 = 20;
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
            println!("Seconds since start: {}", time_since_start);
            if time_since_start > duration_cap_secs {
                break;
            }
            // check detection result
            if msg.motion_detected {
                println!("Motion detected");
            } else {
                println!("No motion detected");
            }
        }
    }
}
