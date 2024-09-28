use std::fs;
use sysinfo::System;

pub fn read_release() -> String {
    let file = "/etc/os-release";

    if let Ok(contents) = fs::read_to_string(file) {
        return contents;
    }

    "OS information not available".to_string()
}

pub fn read_distro_name() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("NAME=") {
                return line.trim_start_matches("NAME=").replace("\"", "").to_string();
            }
        }
    }

    "Unknown Distro".to_string()
}

pub  fn read_osname() -> String {
    let output =  System::long_os_version()
        .unwrap_or_else(|| "Unknown".to_string());

    output
}

pub fn read_hostname() -> String {
    let output = System::host_name()
        .unwrap_or_else(|| "Unknown".to_string());

    output 
}

pub fn read_kernel() -> String {
    let output = System::kernel_version()
        .unwrap_or_else(|| "Unknown".to_string());

    output
}