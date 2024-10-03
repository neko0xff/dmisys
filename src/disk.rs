use std::{
    process::Command,
    fs,
    path::Path
};
use sysinfo::Disks;
use crate::cv;

pub fn run_cmd_smartdata(device:&str) -> String {
    let output = Command::new("smartctl")
        .arg("-a")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn run_cmd_smartstatus(device:&str) -> String {
    let output = Command::new("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn run_cmd_smartinfo(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-i")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn read_disk_smartinfo(device: &str) -> String {
    let output = run_cmd_smartdata(device);
 
     output
 }

pub fn read_disk_smartstatus(device: &str) ->  String {
    let output = run_cmd_smartstatus(device);
    let regex_pattern = r"SMART overall-health self-assessment test result:\s*(\w+)";
    cv::regex_extract(&output,regex_pattern)
}

pub fn read_disk_rotationrate(device: &str) ->  String{
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Rotation Rate:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

pub fn read_disk_devicemodel(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Device Model:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

pub fn read_disk_firmware(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Firmware Version:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

pub fn read_disk_serial(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Serial Number:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

pub fn read_disk_factor(device: &str) -> String { 
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Form Factor:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

pub fn read_disk_sataver(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"SATA Version is:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
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

pub fn read_disk_sectorspace_vec() -> Vec<(String, Option<String>, Option<String>, f64,f64, f64, f64)> {
    let mut disk_info = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let name = disk.name().to_string_lossy().to_string(); // disk name
        let filesystem = disk.file_system().to_str().map(String::from); // file system
        let mount_point = disk.mount_point().to_str().map(String::from); // mount point
        let total_space = cv::bytes_to_gb(disk.total_space()); // total space
        let used_space = cv::bytes_to_gb(disk.total_space() - disk.available_space()); // used space
        let free_space = cv::bytes_to_gb(disk.available_space()); // free space
        let used_point = cv::point_cal(used_space, total_space); // used
       
        if !name.starts_with("overlay") {
            disk_info.push((name, filesystem, mount_point, total_space, used_space,used_point, free_space)); 
        }
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
                            let device_name_str = format!("{}",device_name);
                            let total_size_gb = cv::sectors_to_gb(sectors);
                            disks_info.push((device_name_str, total_size_gb));
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

pub fn read_disks_pyhysicaldrive_vec() -> Vec<(String, f64)> {
    let mut disks_info = Vec::new();
    let block_devices_path = Path::new("/sys/block/");
    
    let block_devices_path = Path::new(block_devices_path);
    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();

                if device_name.starts_with("nvme") ||  device_name.starts_with("sd") || device_name.starts_with("hd") {
                    let size_path = block_devices_path.join(&device_name).join("size");
                    if let Ok(size_str) = fs::read_to_string(size_path) {
                        if let Ok(sectors) = size_str.trim().parse::<u64>() {
                            let device_name_str = format!("/sys/block/{}",device_name);
                            let total_size_gb = cv::sectors_to_gb(sectors);
                            disks_info.push((device_name_str, total_size_gb));
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

pub fn read_disks_physicaldrive_list() -> Vec<String> {
    let mut disks_info = Vec::new();
    let block_devices_path = Path::new("/sys/block/");

    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();
                if device_name.starts_with("nvme") || device_name.starts_with("sd") || device_name.starts_with("hd") {
                    let device_name_str = format!("/dev/{}",device_name);
                    disks_info.push(device_name_str);
                }
            }
        }
    }

    if disks_info.is_empty() {
        disks_info.push("Not Found".to_string());
    }

    disks_info
}

