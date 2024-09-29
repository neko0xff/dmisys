use std::process::Command;
use sysinfo::Disks;
use regex::Regex;
use crate::cv::*;

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

pub fn read_disk_smartstatus(device: &str) ->  String {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"SMART overall-health self-assessment test result:\s*(\w+)").unwrap();

    if let Some(captures) = re.captures(&stdout) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "Unknow".to_string()
    }
}

pub fn read_disk_devicemodel(device: &str) ->  String {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"Device Model:\s*(\w+)").unwrap();

    if let Some(captures) = re.captures(&stdout) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "Unknow".to_string()
    }
}

pub fn read_disk_firmware(device: &str) ->  String {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"Firmware Version:\s*(\w+)").unwrap();

    if let Some(captures) = re.captures(&stdout) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "Unknow".to_string()
    }
}

pub fn read_disk_sataver(device: &str) ->  String {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-H")   
        .arg(device) 
        .output()
        .expect("Failed to execute smartctl");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"SATA Version is:\s*(\w+)").unwrap();

    if let Some(captures) = re.captures(&stdout) {
        captures.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "Unknow".to_string()
    }
}

pub fn read_disk_totalspace() -> (String, f64) {
    let disks = Disks::new_with_refreshed_list();
    let mut name = String::new(); 
    let mut total_space = 0.0; 

    disks.list().into_iter().for_each(|disk| {
        name = disk.name().to_string_lossy().to_string();
        total_space = bytes_to_gb(disk.total_space()) ; 
    });

    if name.is_empty() {
        name = "Not Found".to_string();
        total_space = 0.0;
    }

    (name, total_space)
}