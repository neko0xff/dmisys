use std::{
    process::Command,
    fs,
    path::Path
};
use sysinfo::Disks;
use crate::{
    cv,
    regex::*
};

pub fn read_disk_smartinfo(device: &str) -> Result<String, String> {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-a")
        .arg(device)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                return Err(format!("Failed to execute command"));
            }
            let smart_info = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(smart_info)
        }
        Err(_e) => {
            Err(format!("Please check your install <smartctl> tools. Please ensure you have installed it."))
        }
    }
}

pub fn run_smartstatus(device:&str) -> String {
    let output = Command::new("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn run_smartinfo(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-i")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn read_disk_rotationrate(device: &str) ->  String{
    let output = read_disk_smartinfo(device).unwrap();
    let regex_pattern = r"Rotation Rate:\s*(\d+)";
    extract_info(&output, regex_pattern)
}

pub fn read_disk_smartstatus(device: &str) ->  String {
    let output = run_smartstatus(device);
    let regex_pattern = r"SMART overall-health self-assessment test result:\s*(\w+)";
    extract_info(&output,regex_pattern)
}

pub fn read_disk_devicemodel(device: &str) -> String {
    let output = run_smartinfo(device);
    let regex_pattern = r"Device Model:\s*(.+)";
    extract_info(&output, regex_pattern)
}

pub fn read_disk_firmware(device: &str) -> String {
    let output = run_smartinfo(device);
    let regex_pattern = r"Firmware Version:\s*(.+)";
    extract_info(&output, regex_pattern)
}

pub fn read_disk_sataver(device: &str) -> String {
    let output = run_smartinfo(device);
    let regex_pattern = r"SATA Version is:\s*(.+)";
    extract_info(&output, regex_pattern)
}

pub fn read_disk_totalspace() -> (String, f64) {
    let disks = Disks::new_with_refreshed_list();
    let mut name = String::new(); 
    let mut total_space = 0.0; 

    disks.list().into_iter().for_each(|disk| {
        name = disk.name().to_string_lossy().to_string();
        total_space = cv::bytes_to_gb(disk.total_space()) ; 
    });

    if name.is_empty() {
        name = "Not Found".to_string();
        total_space = 0.0;
    }

    (name, total_space)
}

pub fn read_disk_sectorspace_vec() -> Vec<(String, f64)> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();

    disks.list().into_iter().for_each(|disk| {
        let name = disk.name().to_string_lossy().to_string();
        let total_space = cv::bytes_to_gb(disk.total_space());
        disk_info.push((name, total_space)); 
    });

    if disk_info.is_empty() {
        disk_info.push(("Not Found".to_string(), 0.0));
    }

    disk_info
}

pub fn read_disk_all_vec() -> Vec<(String, f64)> {
    let mut disks_info = Vec::new();
    
    let block_devices_path = Path::new("/sys/block/");
    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();
                
                if !device_name.starts_with("loop") && !device_name.starts_with("ram") {

                    let size_path = block_devices_path.join(&device_name).join("size");
                    if let Ok(size_str) = fs::read_to_string(size_path) {
                        if let Ok(sectors) = size_str.trim().parse::<u64>() {
                            let total_size_gb = cv::sectors_to_gb(sectors);
                            disks_info.push((device_name, total_size_gb));
                        }
                    }
                }
            }
        }
    }

    if disks_info.is_empty() {
        disks_info.push(("Not Found".to_string(), 0.0));
    }

    disks_info
}

pub fn read_disks_pyhysicalhard_vec() -> Vec<(String, f64)> {
    let mut disks_info = Vec::new();
    
    let block_devices_path = Path::new("/sys/block/");
    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();

                if device_name.starts_with("nvme") ||  device_name.starts_with("sd") || device_name.starts_with("hd") {
                    let size_path = block_devices_path.join(&device_name).join("size");
                    if let Ok(size_str) = fs::read_to_string(size_path) {
                        if let Ok(sectors) = size_str.trim().parse::<u64>() {
                            let total_size_gb = cv::sectors_to_gb(sectors);
                            disks_info.push((device_name, total_size_gb));
                        }
                    }
                }
            }
        }
    }

    if disks_info.is_empty() {
        disks_info.push(("Not Found".to_string(), 0.0));
    }

    disks_info
}

pub fn read_disks_physicalhard_list() -> Vec<String> {
    let mut disks_info = Vec::new();
    let block_devices_path = Path::new("/sys/block/");

    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();
                if device_name.starts_with("nvme") || device_name.starts_with("sd") || device_name.starts_with("hd") {
                    disks_info.push(device_name);
                }
            }
        }
    }

    if disks_info.is_empty() {
        disks_info.push("Not Found".to_string());
    }

    disks_info
}