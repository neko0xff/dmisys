use crate::file;
use std::{
    fs,
    process::Command,
    ffi::CStr,
    thread::sleep,
    time::Duration,
    env
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

/// Read: OS release information
/// This function reads the Linux distribution release information from the `/etc/os-release` file.
/// It looks for the line starting with "VERSION=" and returns the value as a `String`.
/// If the line is not found, it returns "Unknown".
pub fn read_release() -> String {
    let file = "/etc/os-release";

    file::read_config_info(file)
}

/// Read: Linux distribution name
/// This function reads the Linux distribution name from the `/etc/os-release` file.
/// It looks for the line starting with "NAME=" and returns the value as a `String`.
/// If the line is not found, it returns "Unknown".
pub fn read_distro_name() -> String {
    let file = "/etc/os-release";
    let find = "NAME=";

    file::read_config_var_string(file, find)
}

/// Read: OS name
/// This function retrieves the operating system name using the `uname` function
/// from the `libc` library. It returns the OS name as a `String`. If the `uname` function fails,
/// it returns "Unknown".
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

/// Read: Hostname
/// This function retrieves the hostname of the system using the `uname` function
/// from the `libc` library. It returns the hostname as a `String`. If the `uname` function fails,
/// it returns "Unknown".
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

/// Read: Kernel Version
/// This function retrieves the kernel version of the operating system
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


/// Read: Disk IO Speed
/// This function calculates the read and write speeds of the disk by reading the number of sectors
/// read and written in the `/proc/diskstats` file. It takes two readings, one before and one after
/// a 1-second sleep, and calculates the difference in megabytes per second.
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

/// Read: System start time
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

/// Read: System start time(UTC)
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
/// This function reads the system initialization process name from the process table
/// and returns it as a String. If the process is not found, it returns "Unknown".
/// The function uses the `ps` command to get the process name of PID 1 (the init process).
/// The `ps` command is executed with the arguments `-p 1 -o comm=` to get the command name
/// of the process with PID 1.
pub fn read_os_init() -> String {
    let output = Command::new("ps")
        .args(&["-p", "1", "-o", "comm="])
        .output();
    
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

/// Read: Terminal type
/// This function reads the terminal type from the environment variable "TERM"
/// and returns it as a String. If the variable is not set, it returns "Unknown".
pub fn read_terminal() -> String {
    env::var("TERM").unwrap_or("Unknown".to_string())
}