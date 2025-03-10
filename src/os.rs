use crate::{
    cv, 
    file
};
use std::{
    fs,
    process::Command,
};
use chrono::DateTime;
use sysinfo::System;

/// Read OS release information
pub fn read_release() -> String {
    let file = "/etc/os-release";

    file::read_config_info(file)
}

/// Read Linux distribution name
pub fn read_distro_name() -> String {
    let file = "/etc/os-release";
    let find = "NAME=";

    file::read_config_var_string(file, find)
}

/// Read OS name
pub fn read_osname() -> String {
    System::long_os_version().unwrap_or_else(|| "Unknown".to_string())
}

/// Read Hostname
pub fn read_hostname() -> String {
    System::host_name().unwrap_or_else(|| "Unknown".to_string())
}

/// Read kernel version
pub fn read_kernel() -> String {
    System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
}

/// Read IO speed (total write and read) in MB/s
pub fn read_io_speed() -> (u64, u64) {
    let mut total_write: u64 = 0;
    let mut total_read: u64 = 0;
    let sys = System::new_all();

    for process in sys.processes().values() {
        let disk_usage = process.disk_usage();
        total_write += cv::bytes_to_mb(disk_usage.written_bytes);
        total_read += cv::bytes_to_mb(disk_usage.read_bytes);
    }

    (total_write, total_read)
}

/// System start time
pub fn system_starttime() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/stat") {
        for line in contents.lines() {
            if let Some(time) = line.strip_prefix("btime ") {
                return time.trim().to_string();
            }
        }
    }
    "Unknown".to_string()
}

/// System start time(UTC)
pub fn system_starttime_utc() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/stat") {
        for line in contents.lines() {
            if let Some(timestamp) = line.strip_prefix("btime ") {
                if let Ok(unix_time) = timestamp.trim().parse::<i64>() {
                    let datetime = DateTime::from_timestamp(unix_time, 0);

                    if let Some(dt) = datetime {
                        return dt.format("%Y-%m-%d  %H:%M:%S").to_string();
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}

/// System init System
pub fn read_os_init() -> String {
    let output = Command::new("ps")
        .args(&["-p", "1", "-o", "comm="])
        .output();
    
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

pub fn read_terminal() -> &'static str {
    env!("TERM")
}