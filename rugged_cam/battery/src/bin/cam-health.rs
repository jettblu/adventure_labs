use log::{ info, error, LevelFilter };
use battery::get_battery_parcentage;
use std::time::SystemTime;
use tokio::time;

#[tokio::main]
pub async fn main() {
    const LATENCY_SECS: u64 = 120;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    // create unique health log file for each boot
    let res = simple_logging::log_to_file(format!("logs/health/{now}.log"), LevelFilter::Info);
    if res.is_err() {
        panic!("Unable to create log file");
    }
    loop {
        let battery_percentage: Result<f32, &'static str> = get_battery_parcentage();
        // seconds since the epoch
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if !battery_percentage.is_ok() {
            // if battery percentage is not fetched, log error
            let error_msg = format!("{} Unable to fetch battery percentage", now);
            error!("{}", error_msg);
        } else {
            let battery_percentage: f32 = battery_percentage.unwrap();
            info!("{} {}", now, battery_percentage);
            time::sleep(time::Duration::from_secs(LATENCY_SECS)).await;
        }
    }
}
