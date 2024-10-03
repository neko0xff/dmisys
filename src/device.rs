use std::{
    process::Command,
    fs,
    path::Path
};
use crate::cv;

/*讀取PCI上的裝置*/
pub fn run_cmd_devicepci() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci")
        .output()
        .expect("Failed to execute command");
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn read_device_info() -> String {
    let output = run_cmd_devicepci();

    output
}

pub fn read_device_gpu() -> Vec<String> {
    let input = read_device_info();
    let regex_pattern =  r"VGA compatible controller:\s*(.+)";
    let output = cv::regex_extract_vec(&input, regex_pattern);

    output
}

pub fn find_devices_counts(device_type: &str) -> Vec<usize> {
    let mut devices = Vec::new();
    let hwmon_dir = Path::new("/sys/class/hwmon");

    if let Ok(entries) = fs::read_dir(hwmon_dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("device/name");
            if let Ok(content) = fs::read_to_string(&path) {
                if content.trim() == device_type {
                    if let Some(hwmon_num) = entry.file_name().to_str()
                        .and_then(|s| s.strip_prefix("hwmon"))
                        .and_then(|s| s.parse().ok())
                    {
                        devices.push(hwmon_num);
                    }
                }
            }
        }
    }
    devices
}