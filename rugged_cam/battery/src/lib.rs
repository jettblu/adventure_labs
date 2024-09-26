use std::process::Command;

pub fn get_battery_parcentage() -> Result<f32, &'static str> {
    const COMMAND: &str = "echo \"get battery\" | nc -q 0 127.0.0.1 8423";
    let res = Command::new("bash")
        .args(["-c", COMMAND])
        .output()
        .expect("failed to execute process");
    let output_string = String::from_utf8_lossy(&res.stdout);
    let output_string = output_string.trim();
    // split line to get battery
    let string_parts = output_string.split(":");
    // let
    for val in string_parts {
        let new_val = val.trim();
        // parse to f32
        let res = new_val.parse::<f32>();
        if res.is_ok() {
            return Ok(res.unwrap());
        }
    }
    Err("Unable to get battery percentage")
}
