use tokio::sync::mpsc;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;
use motion_detection::{ MotionDetector, ParamsMotionDetector, MsgMotionDetector };
use camera_capture::save_photo_to_file;
use log::{ info, trace, warn, LevelFilter };

// example call...
// cargo run -p smart_cam --bin run_smart_cam --release oak 41_40338-2_17403
#[tokio::main]
async fn main() {
    const FILE_COMPRESSED_DIR: &str = "radio/loading_dock/compressed";
    let args: Vec<String> = std::env::args().collect();
    let radio_name = &args[1].to_string();
    // of format... 41_40338-2_17403
    let device_location = &args[2].to_string();
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
                    let msg_time = msg.time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let file_name = &*format!(
                        "radio/loading_dock/original/{}|{}|{}.png",
                        msg_time.clone(),
                        radio_name.clone(),
                        device_location.clone()
                    );
                    let file_name_compressed = format!(
                        "{FILE_COMPRESSED_DIR}/{}|{}|{}.webp",
                        msg_time.clone(),
                        radio_name.clone(),
                        device_location.clone()
                    );
                    save_photo_to_file(file_name, Some(file_name_compressed)).await;
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
