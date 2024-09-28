use std::{
    fs::File,
    io::{self, BufRead}
};
use sysinfo::System;

pub fn get_cpu_model() -> Result<String, io::Error> {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("model name") {
                    let output = line.split(": ")
                            .nth(1)
                            .unwrap_or("")
                            .trim();
                    return Ok(output.to_owned());
                }
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Not found"))
}

pub fn get_cpu_cores() -> Result<i32, io::Error> {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu cores") {
                    let output = line.split(": ").nth(1).unwrap_or("0")
                                .parse::<i32>()
                                .unwrap_or(0);
                    return Ok(output);
                }
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Not found"))
}

pub fn get_cpu_frequency() -> Result<f64, io::Error> {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("cpu MHz") {
                    let mhz = line.split(": ")
                            .nth(1).unwrap_or("0")
                            .parse::<f64>()
                            .unwrap_or(0.0);
                    return Ok(mhz / 1000.0); 
                }
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Not found"))
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