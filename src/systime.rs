use crate::cv;
use libc::sysinfo;
use std::{
    mem::zeroed,
    time::{
        SystemTime, 
        UNIX_EPOCH
    }
};

/// In System Start a time
pub fn read_systime_up() -> (u64, u64, u64) {
    let mut info = unsafe { 
        zeroed::<sysinfo>() 
    };
    unsafe { 
        libc::sysinfo(&mut info) 
    };
    let uptime = info.uptime as u64;
    let (days, hours, minutes) = cv::format_times(uptime);

    (days, hours, minutes)
}

/// In 1970/01/01 start a time(Unix time)
pub fn read_systime_boot() -> (u64, u64, u64) {
    let mut info = unsafe { 
        zeroed::<sysinfo>() 
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    unsafe { 
        libc::sysinfo(&mut info) 
    };
    let uptime = info.uptime as u64;
    let bootime = now - uptime;
    let (days, hours, minutes) = cv::format_times(bootime);

    (days, hours, minutes)
}
