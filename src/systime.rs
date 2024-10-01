use sysinfo::System;
use crate::cv::*;

pub fn read_systime_up() -> (u64,u64,u64) {
    let uptime = System::uptime();
    let (days,hours,minutes)  = format_times(uptime);  

    (days, hours, minutes)
}

pub fn read_systime_boot() -> (u64,u64,u64) {
    let bootime = System::boot_time();
    let (days,hours,minutes)  = format_times(bootime);  

    (days, hours, minutes)
}
