use crate::cv;
use std::{
    fs, 
    path::Path, 
    process::Command
};

/// use command get PCI device list
fn run_cmd_devicepci() -> String {
    let output = Command::new("sh")
        .args(&["-c", "lspci"])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Read use `lspci` a  PCI Device List
/// This function retrieves the PCI device list using the `lspci` command.
/// The device information is then returned as a `String`.
pub fn read_device_info() -> String {
    run_cmd_devicepci()
}

/// Read Installed GPU Device List
/// This function retrieves the installed GPU device list using the `lspci` command.
/// The GPU device information is then returned as a `Vec<String>`.
/// It uses a regular expression to extract the GPU device names from the output.
pub fn read_device_gpu() -> Vec<String> {
    let input = read_device_info();
    let regex_pattern = r"VGA compatible controller:\s*(.+)";

    cv::regex_extract_vec(&input, regex_pattern)
}

/// Find choose a Devices Counts
/// This function searches for devices of a specific type in the `/sys/class/hwmon` directory.
/// It reads the device names from the `device/name` file and checks if they match the specified device type.
pub fn find_devices_counts(device_type: &str) -> Vec<usize> {
    let mut devices = Vec::new();
    let hwmon_dir = Path::new("/sys/class/hwmon");

    if let Ok(entries) = fs::read_dir(hwmon_dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("device/name");
            if let Ok(content) = fs::read_to_string(&path) {
                if content.trim() == device_type {
                    if let Some(hwmon_num) = entry
                        .file_name()
                        .to_str()
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

/// Read Power Supply Device Counts
/// This function counts the number of power supply devices in the `/sys/class/power_supply` directory.
/// It looks for entries that start with "ADP" or "BAT" and increments the count accordingly.
/// The function returns the total count of power supply devices.
pub fn read_adp_counts() -> usize {
    let power_supply_dir = "/sys/class/power_supply/";
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(power_supply_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap_or_default();
                if device_name.starts_with("ADP") {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Read Battery Device Counts
/// This function counts the number of battery devices in the `/sys/class/power_supply` directory.
/// It looks for entries that start with "BAT" and increments the count accordingly.
/// The function returns the total count of battery devices.
pub fn read_bat_counts() -> usize {
    let power_supply_dir = "/sys/class/power_supply/";
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(power_supply_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap_or_default();
                if device_name.starts_with("BAT") {
                    count += 1;
                }
            }
        }
    }

    count
}
