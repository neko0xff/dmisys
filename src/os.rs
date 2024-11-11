use crate::{cv, file};
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
