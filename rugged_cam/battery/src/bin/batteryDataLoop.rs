use log::{ info, LevelFilter };
use battery::get_battery_parcentage;
use std::time::SystemTime;
use tokio::time;

#[tokio::main]
pub async fn main() {
    const LATENCY_SECS: u64 = 120;
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    // create unique health log file for each boot
    let _ = simple_logging::log_to_file(format!("logs/health/{now}.log"), LevelFilter::Info);
    loop {
        let battery_percentage: Result<f32, &'static str> = get_battery_parcentage();
        if !battery_percentage.is_ok() {
            panic!("Unable to fetch battery percentage");
        }
        let battery_percentage: f32 = battery_percentage.unwrap();
        // seconds since the epoch
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        info!("{} {}", now, battery_percentage);
        time::sleep(time::Duration::from_secs(LATENCY_SECS)).await;
    }
}
