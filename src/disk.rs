use crate::cv;
use std::{
    fs,
    path::Path,
    process::Command,
    ffi::CString,
    mem::MaybeUninit,
    io::{
        BufRead,
        BufReader
    },
    fs::File,
};
use libc::statvfs;

/// read disk info
fn run_cmd_smartdata(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-a")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// read `samrtctl` command a data
fn run_cmd_smartstatus(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-H")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_cmd_smartinfo(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-i")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// read this pyhysical disk information
pub fn read_disk_smartinfo(device: &str) -> String {
    run_cmd_smartdata(device)
}

/// read this pyhysical disk S.M.A.R.T status
pub fn read_disk_smartstatus(device: &str) -> String {
    let output = run_cmd_smartstatus(device);
    let regex_pattern = r"SMART overall-health self-assessment test result:\s*(\w+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk Rotation Rate
pub fn read_disk_rotationrate(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Rotation Rate:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk device model
pub fn read_disk_devicemodel(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Device Model:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk firmware version
pub fn read_disk_firmware(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Firmware Version:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk serial number
pub fn read_disk_serial(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Serial Number:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk factor
pub fn read_disk_factor(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Form Factor:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk sata version
pub fn read_disk_sataver(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"SATA Version is:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// read this pyhysical disk total space
fn get_disk_size(mount_point: &str) -> Option<f64> {
    let path = CString::new(mount_point).unwrap();
    let mut stat: MaybeUninit<statvfs> = MaybeUninit::uninit();

    unsafe {
        if statvfs(path.as_ptr(), stat.as_mut_ptr()) == 0 {
            let stat = stat.assume_init();
            let total_size = stat.f_blocks as u64 * stat.f_frsize as u64;

            return Some(cv::bytes_to_gb(total_size)); 
        }
    }


    None
}

/// read this pyhysical disk total space
pub fn read_disk_totalspace() -> (String, f64) {
    let path = "/proc/mounts";
    let file = File::open(path).expect("Failed to open /proc/mounts");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                let device_name = parts[0].to_string(); // Disk Name
                let mount_point = parts[1]; // Disk Mount Point

                if let Some(total_space) = get_disk_size(mount_point) {
                    return (device_name, total_space);
                }
            }
        }
    }

    ("Not Found".to_string(), 0.0)
}



/// Read disk sector space and return as map
pub fn get_disk_info(mount_point: &str) -> Option<(f64, f64, f64, f64)> {
    let path = CString::new(mount_point).unwrap();
    let mut stat: MaybeUninit<statvfs> = MaybeUninit::uninit();

    unsafe {
        if statvfs(path.as_ptr(), stat.as_mut_ptr()) == 0 {
            let stat = stat.assume_init();
            let total_space = stat.f_blocks as u64 * stat.f_frsize as u64;
            let free_space = stat.f_bavail as u64 * stat.f_frsize as u64;
            let used_space = total_space - free_space;

            let total_gb = cv::bytes_to_gb(total_space);
            let used_gb = cv::bytes_to_gb(used_space);
            let free_gb = cv::bytes_to_gb(free_space);
            let used_percent = (used_gb / total_gb) * 100.0;

            return Some((total_gb, used_gb, used_percent, free_gb));
        }
    }
    None
}

/// Read disk sector space and return as vector
pub fn read_disk_sectorspace_vec() -> Vec<(String, Option<String>, Option<String>, f64, f64, f64, f64)> {
    let file = File::open("/proc/mounts").expect("Failed to open /proc/mounts");
    let reader = BufReader::new(file);
    let mut disk_info = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                let name = parts[0].to_string(); // Disk name
                let mount_point = parts[1].to_string(); // Disk mount point
                let filesystem = parts[2].to_string(); // Disk File System

                if let Some((total_space, used_space, used_percent, free_space)) = get_disk_info(&mount_point) {
                    disk_info.push((
                        name,
                        Some(filesystem),
                        Some(mount_point),
                        total_space,
                        used_space,
                        used_percent,
                        free_space,
                    ));
                }
            }
        }
    }

    disk_info
}

/// Read PyhysicalDrive info and return as vector
pub fn read_disks_pyhysicaldrive_vec() -> Vec<(String, f64)> {
    let mut disks_info = Vec::new();
    let block_devices_path = Path::new("/sys/block/");

    let block_devices_path = Path::new(block_devices_path);
    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();

                if device_name.starts_with("nvme")
                    || device_name.starts_with("sd")
                    || device_name.starts_with("hd")
                {
                    let size_path = block_devices_path.join(&device_name).join("size");
                    if let Ok(size_str) = fs::read_to_string(size_path) {
                        if let Ok(sectors) = size_str.trim().parse::<u64>() {
                            let device_name_str = format!("/sys/block/{}", device_name);
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

/// Read PyhysicalDrive info and return as list
pub fn read_disks_physicaldrive_list() -> Vec<String> {
    let mut disks_info = Vec::new();
    let block_devices_path = Path::new("/sys/block/");

    if let Ok(entries) = fs::read_dir(block_devices_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let device_name = entry.file_name().into_string().unwrap();
                if device_name.starts_with("nvme")
                    || device_name.starts_with("sd")
                    || device_name.starts_with("hd")
                {
                    let device_name_str = format!("/dev/{}", device_name);
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

/// Read disk sector space and return as map
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
                            let device_name_str = device_name.to_string();
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