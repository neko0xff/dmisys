use sysinfo::System;
use crate::cv::*;

pub fn read_systime_uptime() -> (u64,u64,u64) {
    let uptime = System::uptime();
    let days = sec_to_day(uptime);
    let hours = sec_to_hours(uptime);
    let minutes = sec_to_mins(uptime);

    (days, hours, minutes)
}