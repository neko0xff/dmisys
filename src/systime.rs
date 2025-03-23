use crate::cv;
use libc::sysinfo;
use std::{
    mem::zeroed,
    time::{
        SystemTime, 
        UNIX_EPOCH
    }
};

/// Get Linux C sysinfo Libary
fn get_libc_sysinfo() -> sysinfo {
    let mut info = unsafe { zeroed::<sysinfo>() };
    unsafe {
        sysinfo(&mut info);
    }

    info
}

/// systime: In System Start a time
pub fn read_systime_up() -> (u64, u64, u64) {
    let uptime = get_libc_sysinfo().uptime as u64;
    let (days, hours, minutes) = cv::format_times(uptime);

    (days, hours, minutes)
}

/// systime: In 1970/01/01 start a time(Unix time)
pub fn read_systime_boot() -> (u64, u64, u64) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let uptime = get_libc_sysinfo().uptime as u64;
    let bootime = now - uptime;
    let (days, hours, minutes) = cv::format_times(bootime);

    (days, hours, minutes)
}
