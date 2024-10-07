use sysinfo::System;
use crate::cv::*;

/// time: In System Start
pub fn read_systime_up() -> (u64,u64,u64) {
    let uptime = System::uptime();
    let (days,hours,minutes)  = format_times(uptime);  

    (days, hours, minutes)
}

/// time: In 1970/01/01 start a time(Unix time)
pub fn read_systime_boot() -> (u64,u64,u64) {
    let bootime = System::boot_time();
    let (days,hours,minutes)  = format_times(bootime);  

    (days, hours, minutes)
}
