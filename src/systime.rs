use sysinfo::System;
use crate::cv;

/// In System Start a time
pub fn read_systime_up() -> (u64,u64,u64) {
    let uptime = System::uptime();
    let (days,hours,minutes)  = cv::format_times(uptime);  

    (days, hours, minutes)
}

/// In 1970/01/01 start a time(Unix time)
pub fn read_systime_boot() -> (u64,u64,u64) {
    let bootime = System::boot_time();
    let (days,hours,minutes)  = cv::format_times(bootime);  

    (days, hours, minutes)
}
