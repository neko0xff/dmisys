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

/// Read:  disk info
fn run_cmd_smartdata(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-a")
        .arg(device)
        .output()
        .expect("Failed to execute smartctl");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Read:  `samrtctl` command a data
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

/// Read:  this pyhysical disk information
/// This function retrieves the S.M.A.R.T. information of a physical disk.
/// It uses the `smartctl` command to get the information and returns it as a string.
/// The function takes a device path as an argument, which is typically in the format `/dev/sdX` or `/dev/nvmeXnY`.
/// The function uses the `Command` struct from the `std::process` module to execute the command and capture its output.
pub fn read_disk_smartinfo(device: &str) -> String {
    run_cmd_smartdata(device)
}

/// Read:  this pyhysical disk S.M.A.R.T status
/// This function retrieves the S.M.A.R.T. status of a physical disk.
/// It uses the `smartctl` command to get the status and returns it as a string.
/// The function takes a device path as an argument, which is typically in the format `/dev/sdX` or `/dev/nvmeXnY`.
pub fn read_disk_smartstatus(device: &str) -> String {
    let output = run_cmd_smartstatus(device);
    let regex_pattern = r"SMART overall-health self-assessment test result:\s*(\w+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk Rotation Rate
/// This function retrieves the rotation rate of a physical disk.
/// The function returns the extracted rotation rate as a string.
/// If the rotation rate is not found, it returns an empty string.
pub fn read_disk_rotationrate(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Rotation Rate:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk device model
/// This function retrieves the device model of a physical disk.
/// The function returns the extracted device model as a string.
/// If the device model is not found, it returns an empty string.
pub fn read_disk_devicemodel(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Device Model:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk firmware version
/// This function retrieves the firmware version of a physical disk.
/// The function returns the extracted firmware version as a string.
/// If the firmware version is not found, it returns an empty string.
pub fn read_disk_firmware(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Firmware Version:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk serial number
/// This function retrieves the serial number of a physical disk.
/// The function returns the extracted serial number as a string.
/// If the serial number is not found, it returns an empty string.
/// The serial number is typically a unique identifier for the disk.
/// It is useful for identifying and managing disks in a system.
pub fn read_disk_serial(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Serial Number:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk factor
/// This function retrieves the form factor of a physical disk.
/// The form factor indicates the physical size and shape of the disk.
/// The function returns the extracted form factor as a string.
/// If the form factor is not found, it returns an empty string.
/// The form factor is important for compatibility with different hardware configurations.
/// It helps determine if the disk can fit into a specific slot or enclosure.
/// The form factor can vary between different types of disks, such as 2.5-inch, 3.5-inch, or M.2.
pub fn read_disk_factor(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"Form Factor:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk sata version
/// This function retrieves the SATA version of a physical disk.
/// The SATA version indicates the interface standard used by the disk.
/// The function returns the extracted SATA version as a string.
/// If the SATA version is not found, it returns an empty string.
/// The SATA version is important for compatibility with different hardware configurations.
/// It helps determine the maximum data transfer rates and features supported by the disk.
/// The SATA version can vary between different generations, such as SATA I, SATA II, or SATA III.
pub fn read_disk_sataver(device: &str) -> String {
    let output = run_cmd_smartinfo(device);
    let regex_pattern = r"SATA Version is:\s*(.+)";
    cv::regex_extract(&output, regex_pattern)
}

/// Read:  this pyhysical disk total space
/// This function retrieves the total space of a physical disk.
/// It uses the `statvfs` system call to get the file system statistics.
/// The function takes a mount point as an argument, which is typically the directory where the disk is mounted.
/// The function returns the total space in gigabytes (GB).
/// The total space is calculated by multiplying the number of blocks by the block size.
/// The function returns an `Option<f64>` to indicate success or failure.
/// If the total space is successfully retrieved, it returns `Some(total_space)`.
/// If the total space cannot be retrieved, it returns `None`.
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

/// Read:  this pyhysical disk total space
/// This function retrieves the total space of a physical disk.
/// It uses the `statvfs` system call to get the file system statistics.
/// The function takes a mount point as an argument, which is typically the directory where the disk is mounted.
/// The function returns the total space in gigabytes (GB).
/// The total space is calculated by multiplying the number of blocks by the block size.
/// The function returns an `Option<f64>` to indicate success or failure.
/// If the total space is successfully retrieved, it returns `Some(total_space)`.
/// If the total space cannot be retrieved, it returns `None`.
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



/// Read:  disk sector space and return as map
/// This function retrieves the disk space information for a given mount point.
/// It uses the `statvfs` system call to get the file system statistics.
/// The function takes a mount point as an argument, which is typically the directory where the disk is mounted.
/// The function returns a tuple containing the total space, used space, used percentage, and free space.
/// The total space is calculated by multiplying the number of blocks by the block size.
/// The used space is calculated by subtracting the free space from the total space.
/// The used percentage is calculated by dividing the used space by the total space and multiplying by 100.
/// The function returns an `Option<(f64, f64, f64, f64)>` to indicate success or failure.
/// If the disk space information is successfully retrieved, it returns `Some((total_space, used_space, used_percent, free_space))`.
/// If the disk space information cannot be retrieved, it returns `None`.
/// The total space, used space, and free space are returned in gigabytes (GB).
/// The used percentage is returned as a percentage value.
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

/// Read:  disk sector space and return as vector
/// This function retrieves the disk space information for all mounted disks.
/// It reads the `/proc/mounts` file to get the list of mounted disks.
/// For each mounted disk, it retrieves the total space, used space, used percentage, and free space.
/// The function returns a vector of tuples, where each tuple contains the disk name, file system type,
/// mount point, total space, used space, used percentage, and free space.
/// The total space, used space, and free space are returned in gigabytes (GB).
/// The used percentage is returned as a percentage value.
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

/// Read:  PyhysicalDrive info and return as vector
/// This function retrieves the physical drive information for all block devices.
/// It reads the `/sys/block/` directory to get the list of block devices.
/// For each block device, it retrieves the device name and total size in gigabytes.
/// The function returns a vector of tuples, where each tuple contains the device name and total size.
/// The total size is calculated by reading the `size` file in the block device directory.
/// The total size is returned in gigabytes (GB).
/// The function filters the block devices to include only those that start with "nvme", "sd", or "hd".
/// If no block devices are found, it returns a vector with a single tuple containing "Not Found" and 0.0.
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

/// Read:  PyhysicalDrive info and return as list
/// This function retrieves the physical drive information for all block devices.
/// It reads the `/sys/block/` directory to get the list of block devices.
/// For each block device, it retrieves the device name.
/// The function returns a vector of strings, where each string is the device name.
/// The function filters the block devices to include only those that start with "nvme", "sd", or "hd".
/// If no block devices are found, it returns a vector with a single string "Not Found".
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

/// Read:  disk sector space and return as map
/// This function retrieves the disk space information for a given mount point.
/// It uses the `statvfs` system call to get the file system statistics.
/// The function takes a mount point as an argument, which is typically the directory where the disk is mounted.
/// The function returns a tuple containing the total space, used space, used percentage, and free space.
/// The total space is calculated by multiplying the number of blocks by the block size.
/// The used space is calculated by subtracting the free space from the total space.
/// The used percentage is calculated by dividing the used space by the total space and multiplying by 100.
/// The function returns an `Option<(f64, f64, f64, f64)>` to indicate success or failure.
/// If the disk space information is successfully retrieved, it returns `Some((total_space, used_space, used_percent, free_space))`.
/// If the disk space information cannot be retrieved, it returns `None`.
/// The total space, used space, and free space are returned in gigabytes (GB).
/// The used percentage is returned as a percentage value.
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