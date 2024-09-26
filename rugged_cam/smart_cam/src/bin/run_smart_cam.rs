use tokio::sync::mpsc;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use motion_detection::{ MotionDetector, ParamsMotionDetector, MsgMotionDetector };
use camera_capture::{ save_photo_to_file };
use log::{ info, trace, warn, LevelFilter };

#[tokio::main]
async fn main() {
    simple_logging::log_to_file("logs/smart_cam/test.log", LevelFilter::Info);
    info!("Smart cam program started");
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
    let duration_cap_secs: u64 = 20;
    let start_time: SystemTime = SystemTime::now();
    let mut motion_session_captured: bool = false;
    let mut frame_count: u64 = 0;
    // to do... add command line argument for max duration or infinite loop
    let max_frame_count = 50;
    loop {
        if frame_count >= max_frame_count {
            println!("Requested frame count achieved. Exiting loop...");
            break;
        }
        let val: Option<MsgMotionDetector> = rx.recv().await;
        if val.is_none() {
            println!("No value received");
        } else {
            let msg = val.unwrap();
            // let time_since_start = msg.time
            //     .duration_since(start_time.clone())
            //     .expect("Invalid time operation")
            //     .as_secs();
            // if time_since_start > duration_cap_secs {
            //     break;
            // }
            // check detection result
            if msg.motion_detected {
                if !motion_session_captured {
                    // to do... add error handling for photo capture
                    let file_name = &*format!("smart_cam_test_output/{frame_count} frame.png");
                    save_photo_to_file(file_name);
                    // toggle capture indicator
                    motion_session_captured = true;
                    frame_count = frame_count + 1;
                }
                println!("Motion detected");
            } else {
                motion_session_captured = false;
                println!("No motion detected");
            }
        }
    }
    println!("Smart cam program done.")
}
