use sysinfo:: System;
use crate::cv::*;

pub fn total_memory() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.total_memory()) ;

    output
}

pub fn used_memory() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.used_memory());

    output
}

pub fn free_memory() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.free_memory());

    output
}

pub fn total_swap() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.total_swap());

    output
}

pub fn free_swap() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.free_swap());

    output
}

pub fn used_swap() -> u64{
    let sys = System::new_all();
    let output = bytes_to_mb(sys.used_swap());

    output
}
