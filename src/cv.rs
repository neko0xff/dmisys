use regex::Regex;
use std::error::Error;

/// bytes conversion: Giga Bytes(GB)
pub fn bytes_to_gb(bytes: u64) -> f64 {
    let cv_gb = f64::powf(2_f64, 30_f64);

    (bytes as f64) / cv_gb
}

/// bytes conversion: Mega Bytes(MB)
pub fn bytes_to_mb(data: u64) -> u64 {
    let cv_mb = u64::pow(2, 20);

    data / cv_mb
}

/// SECTORS to Giga Bytes(GB)
pub fn sectors_to_gb(data: u64) -> f64 {
    (data * 512) as f64 / 1_073_741_824.0
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
