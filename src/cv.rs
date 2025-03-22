use regex::Regex;
use std::error::Error;

/// bytes conversion: Giga Bytes(GB)
pub fn bytes_to_gb(bytes: u64) -> f64 {
    const GB_CONVERSION: f64 = 1_073_741_824.0; // 1 GB = 2^30 bytes
    bytes as f64 / GB_CONVERSION
}

/// bytes conversion: Mega Bytes(MB)
pub fn bytes_to_mb(bytes: u64) -> f64 {
    const MB_CONVERSION: f64 = 1_048_576.0;  // 1 MB = 2^20 bytes
    bytes as f64 / MB_CONVERSION
}

/// SECTORS to Giga Bytes(GB)
pub fn sectors_to_gb(sectors: u64) -> f64 {
    const SECTOR_SIZE: u64 = 512; // 1 sector = 512 bytes
    bytes_to_gb(sectors * SECTOR_SIZE)
}

/// time conversionsecord: sec to day
pub fn sec_to_day(data: u64) -> u64 {
    data / 86400
}

/// time conversionsecord: sec to hours
pub fn sec_to_hours(data: u64) -> u64 {
    (data % 86400) / 3600
}

/// time conversionsecord: sec to mins
pub fn sec_to_mins(data: u64) -> u64 {
    (data % 3600) / 60
}

/// percentage calculation
pub fn percentage_cal(used: f64, total: f64) -> f64 {
    (used / total) * 100.0
}

/// Unit Conversion: Converts millivolts (mV) to volts (V).
pub fn mv_to_volts(mv: usize) -> f64 {
    (mv as f64) / 1_000.0
}

/// Unit conversion: Convert milliampere hours (mAh) to microampere hours (μAh).
pub fn mah_to_uah(mah: usize) -> f64 {
    (mah as f64) * 1_000.0
}

/// Unit Conversion: Converts milliamps (mA) to amperes (A).
pub fn ma_to_a(ma: usize) -> f64 {
    (ma as f64) / 1_000.0
}

/// Unit Conversion: Converts micro ampere hours (μAh) to milli ampere hours (mAh).
pub fn uah_to_mah(uah: usize) -> f64 {
    (uah as f64) / 1_000.0
}

/// Unit conversion: Converts microamps (μA) to milliamps (mA).
pub fn ua_to_ma(ua: usize) -> f64 {
    (ua as f64) / 1_000.0
}

/// Unit Conversion: Converts microvolts (µV) to volts (V).
pub fn uv_to_volts(uv: usize) -> f64 {
    (uv as f64) / 1_000_000.0
}

/// Unit conversion: Converts milliamps (mA) to microamps (μA).
pub fn ma_to_ua(ma: usize) -> f64 {
    (ma as f64) * 1_000.0
}

/// Unit conversion: Converts ns to sec
pub fn ns_to_sec(time:u64) -> u64 {
    time/ 1_000_000
}

/// data struct: usize to bool
pub fn usize_to_bool(n: usize) -> bool {
    let mut out = false;

    if n == 0 {
        out = false;
    } else if n == 1 {
        out = true;
    }

    out
}

///  data struct: bool to usize
pub fn bool_to_usize(n: bool) -> usize {
    let mut out: usize = 0;

    if !n {
        out = 0;
    } else if n {
        out = 1;
    }

    out
}

/// format : time(days,hour,minutes)
pub fn format_times(time: u64) -> (u64, u64, u64) {
    let days = sec_to_day(time);
    let hours = sec_to_hours(time);
    let minutes = sec_to_mins(time);

    (days, hours, minutes)
}

/// format : time(hour,minutes,seconds)
pub fn sec_to_playingtime(time:u64) -> (u64,u64,u64) {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;

    (hours,minutes,seconds)
}

/// format: message
pub fn format_msg(result: Result<Option<String>, Box<dyn Error>>) -> String {
    match result {
        Ok(Some(data)) => data,
        Ok(None) => "No Found".to_string(),
        Err(e) => format!("ERROR={}", e),
    }
}

/// regex a once data
pub fn regex_extract(input: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();

    if let Some(captures) = re.captures(input) {
        captures
            .get(1)
            .map_or("Unknown", |m| m.as_str())
            .to_string()
    } else {
        "Unknown".to_string()
    }
}

/// regex a list
pub fn regex_extract_vec(output: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    let output: Vec<String> = re
        .captures_iter(output)
        .map(|cap| cap[1].trim().to_string())
        .collect();

    output
}
