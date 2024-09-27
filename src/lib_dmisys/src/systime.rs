use sysinfo::System;
use crate::cv::*;

pub read systime_uptime() -> (u64,u64,u64) {
    let uptime = System::uptime();
    let days = cv::sec_to_day(uptime);
    let hours = cv::sec_to_hours(uptime);
    let minutes = cv::sec_to_mins(uptime);

    (days, hours, minutes)
}