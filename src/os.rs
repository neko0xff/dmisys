use sysinfo::System;
use crate::{cv,file};

pub fn read_release() -> String {
    let file = "/etc/os-release";
    let output = file::read_config_info(file);

    output
}

pub fn read_distro_name() -> String {
    let file = "/etc/os-release";
    let find = "NAME=";
    let output = file::read_config_var_string(file, find);

    output
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

pub fn read_io_speed() -> (u64, u64) {
    let mut total_write: u64 = 0;
    let mut total_read: u64 = 0;
    let sys = System::new_all();  

    for (_pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        total_write += cv::bytes_to_mb(disk_usage.written_bytes);
        total_read +=  cv::bytes_to_mb(disk_usage.read_bytes);
    }

    (total_write, total_read)
}