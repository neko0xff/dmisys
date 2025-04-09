use std::{
    fs,
    fs::File,
    io::{
        self,
        BufRead
    },
    ffi::CStr
};
use libc::{
    uname, 
    utsname,
    getloadavg,
    sysconf, 
    _SC_NPROCESSORS_ONLN
};

/// CPU Model
/// This function reads the CPU model information from the `/proc/cpuinfo` file.
/// It looks for the line starting with "model name" and returns the value as a `String`.
/// If the line is not found, it returns "Unknown".
pub fn read_cpu_model() -> String {
    let info_path = "/proc/cpuinfo";

    if let Ok(contents) = fs::read_to_string(info_path) {
        for line in contents.lines() {
            if line.starts_with("model name") {
                let output = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim()
                    .to_string();
                return output;
            } else {
                
            }
        }
    }
    
    "Unknown".to_string()
}

/// CPU Cores
/// This function reads the number of CPU cores from the `/proc/cpuinfo` file.
/// It looks for the line starting with "cpu cores" and returns the value as a `u64`.
/// If the line is not found, it returns 0.
pub fn read_cpu_cores() -> u64 {
    let info_path = "/proc/cpuinfo";

    if let Ok(file) = File::open(info_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu cores") {
                    let output = line
                        .split(": ")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                    return output;
                } else {
                    
                }
            }
        }
    }

   0
}

/// CPU Threads
/// This function reads the number of CPU threads from the `/proc/cpuinfo` file.
/// It looks for the line starting with "siblings" and returns the value as a `u64`.
/// If the line is not found, it returns 0.
pub fn read_cpu_threads() -> u64 {
    let info_path = "/proc/cpuinfo";

    if let Ok(file) = File::open(info_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("siblings") {
                    let output = line
                        .split(": ")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                    return output;
                } else {
                   
                }
            }
        }
    }

    0
}

/// CPU Frequency (MHz)
/// This function reads the CPU frequency information from the `/proc/cpuinfo` file.
/// It looks for the line starting with "cpu MHz" and returns the value as a `f64`.
/// If the line is not found, it returns 0.0.
pub fn get_cpu_frequency() -> f64 {
    let info_path = "/proc/cpuinfo";

    if let Ok(file) = File::open(info_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu MHz") {
                    let mhz = line
                        .split(": ")
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    let output = mhz / 1000.0;
                    return output;
                } else {
                    
                }
            }
        }
    }

    0.0
}

/// CPU Load Average (1 minutes)
/// This function retrieves the CPU load average for the last 1 minute.
/// It returns the load average as a `f64` value.
pub fn get_cpu_loading() -> f64 {
    let mut loads: [f64; 1] = [0.0]; 
    let cpu_cores = unsafe { 
        sysconf(_SC_NPROCESSORS_ONLN) 
    } as f64; 

    unsafe {
        if getloadavg(loads.as_mut_ptr(), 1) == 1 {
            loads[0] / cpu_cores * 100.0
        } else {
            0.0
        }
    }
}

/// CPU Architecture
///  This function retrieves the CPU architecture information using the `uname` function
/// /// It returns the CPU architecture as a `String`. If the `uname` function fails,
/// /// it returns "Unknown".
pub fn read_cpu_arch() -> String {
    unsafe {
        let mut uts = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65], 
            domainname: [0; 65],
        };

        if uname(&mut uts) == 0 {
            CStr::from_ptr(uts.machine.as_ptr()).to_string_lossy().into_owned() // cpu architecture
        } else {
            "Unknown".to_string()
        }
    }
}
