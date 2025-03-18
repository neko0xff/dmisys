use crate::file;
use std::{
    fs,
    process::Command,
    ffi::CStr,
    thread::sleep,
    time::Duration
};
use libc::{
    uname, 
    utsname
};
use chrono::DateTime;

/// Disk IO read & write
fn read_dir_disk() -> (u64, u64) {
    let data = fs::read_to_string("/proc/diskstats");
    let mut total_read = 0u64;
    let mut total_write = 0u64;
    for line in data.expect("REASON").lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 14 {
            continue;
        }
        let device_name = parts[2];
        if !device_name.contains("loop") {
            total_read += parts[5].parse::<u64>().unwrap_or(0); 
            total_write += parts[9].parse::<u64>().unwrap_or(0); 
        }
    }

    (total_read, total_write)
}

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
    unsafe {
        let mut uts = utsname {
            sysname: [0; 65], 
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };

        if uname(&mut uts) == 0 {
            CStr::from_ptr(uts.sysname.as_ptr()).to_string_lossy().into_owned() //os name
        } else {
            "Unknown".to_string()
        } 
    }
}

/// Read Hostname
pub fn read_hostname() -> String {
    unsafe {
        let mut uts = utsname {
            sysname: [0; 65], 
            nodename: [0; 65], 
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };

        if uname(&mut uts) == 0 {
            CStr::from_ptr(uts.nodename.as_ptr()).to_string_lossy().into_owned() // host name
        } else {
            "Unknown".to_string()
        }
    }
}

/// Read kernel version
pub fn read_kernel() -> String {
    unsafe {
        let mut uts = utsname {
            sysname: [0; 65], 
            nodename: [0; 65], 
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };

        if uname(&mut uts) == 0 {
            CStr::from_ptr(uts.release.as_ptr()).to_string_lossy().into_owned() // kernel version
        } else {
            "Unknown".to_string()
        }
    }
}


/// Disk IO Speed
/// test cmd: $ dd if=/dev/zero of=testfile bs=1M count=10000 && sync
pub fn read_io_speed() -> (f64, f64) {
    let sector_size = 512.0;
    let (read_sectors_1, write_sectors_1) = read_dir_disk(); // 1 time
    sleep(Duration::from_secs(1)); // sleep 1 sec
    let (read_sectors_2, write_sectors_2) = read_dir_disk(); // 2 time
    
    let read_mb = ((read_sectors_2.saturating_sub(read_sectors_1)) as f64 * sector_size) / (1024.0 * 1024.0);
    let write_mb = ((write_sectors_2.saturating_sub(write_sectors_1)) as f64 * sector_size) / (1024.0 * 1024.0);
    
  (read_mb, write_mb)
}

/// System start time
pub fn system_starttime() -> String {
    let path =  "/proc/stat";

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if let Some(time) = line.strip_prefix("btime ") {
                return time.trim().to_string();
            } else {
                
            }
        }
    }

    "Unknown".to_string()
}

/// System start time(UTC)
pub fn system_starttime_utc() -> String {
    let path =  "/proc/stat";

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if let Some(timestamp) = line.strip_prefix("btime ") {
                // to UTC time
                if let Ok(unix_time) = timestamp.trim().parse::<i64>() {
                    let datetime = DateTime::from_timestamp(unix_time, 0);

                    if let Some(dt) = datetime {
                        return dt.format("%Y-%m-%d  %H:%M:%S").to_string();
                    } else {

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