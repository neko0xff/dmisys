use std::{
    fs::File,
    io::{self, BufRead}
};
use sysinfo::System;

pub fn read_cpu_model() -> String {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("model name") {
                    let get = line.split(": ")
                            .nth(1)
                            .unwrap_or("")
                            .trim();
                    let output = get.to_owned();
                    return output;
                }
            }
        }
    }
    
    "Unknown".to_string()
}

pub fn read_cpu_cores() -> u64 {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu cores") {
                    let output = line.split(": ")
                            .nth(1)
                            .unwrap_or("0")
                            .parse::<u64>()
                            .unwrap_or(0);
                    return output;
                }
            }
        }
    }

    0
}

pub fn read_cpu_threads() -> u64 {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("siblings") {
                    let output = line.split(": ")
                            .nth(1)
                            .unwrap_or("0")
                            .parse::<u64>()
                            .unwrap_or(0);
                    return output;
                }
            }
        }
    }

    0
}

pub fn get_cpu_frequency() -> f64 {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu MHz") {
                    let mhz = line.split(": ")
                            .nth(1)
                            .unwrap_or("0")
                            .parse::<f64>()
                            .unwrap_or(0.0);
                    let output = mhz / 1000.0; 
                    return output; 
                }
            }
        }
    }
    
    0.0
}

pub fn get_cpu_loading() -> f64 {
    // 回傳部分: 讀取1分鐘內的系統負戴值
    let read_cpu_loading = System::load_average();
    let output = read_cpu_loading.one; 

    output
}

pub fn read_cpu_arch() -> String {
    let output = System::cpu_arch()
            .unwrap_or_else(|| "Unknown".to_string());

    output
}
