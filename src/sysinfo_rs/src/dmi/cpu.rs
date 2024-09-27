use std::fs::File;
use std::io::{self, BufRead};

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

