use regex::Regex;
use std::error::Error;

/// bytes conversion: Giga Bytes(GB)
/// This function converts bytes to gigabytes.
/// It takes a `u64` value representing the number of bytes and returns a `f64` value representing the equivalent in gigabytes.
/// It uses a constant conversion factor of 1 GB = 2^30 bytes.
pub fn bytes_to_gb(bytes: u64) -> f64 {
    const GB_CONVERSION: f64 = 1_073_741_824.0; // 1 GB = 2^30 bytes
    bytes as f64 / GB_CONVERSION
}

/// bytes conversion: Mega Bytes(MB)
/// This function converts bytes to megabytes.
/// It takes a `u64` value representing the number of bytes and returns a `f64` value representing the equivalent in megabytes.
/// It uses a constant conversion factor of 1 MB = 2^20 bytes.
pub fn bytes_to_mb(bytes: u64) -> f64 {
    const MB_CONVERSION: f64 = 1_048_576.0;  // 1 MB = 2^20 bytes
    bytes as f64 / MB_CONVERSION
}

/// SECTORS to Giga Bytes(GB)
/// This function converts sectors to gigabytes.
/// It takes a `u64` value representing the number of sectors and returns a `f64` value representing the equivalent in gigabytes.
/// It uses a constant conversion factor of 1 sector = 512 bytes.
pub fn sectors_to_gb(sectors: u64) -> f64 {
    const SECTOR_SIZE: u64 = 512; // 1 sector = 512 bytes
    bytes_to_gb(sectors * SECTOR_SIZE)
}

/// time conversionsecord: sec to day
/// This function converts seconds to days.
/// It takes a `u64` value representing the number of seconds and returns a `u64` value representing the equivalent in days.
/// It uses a constant conversion factor of 1 day = 86400 seconds.
pub fn sec_to_day(data: u64) -> u64 {
    data / 86400
}

/// time conversionsecord: sec to hours
/// This function converts seconds to hours.
/// It takes a `u64` value representing the number of seconds and returns a `u64` value representing the equivalent in hours.
/// It uses a constant conversion factor of 1 hour = 3600 seconds.
pub fn sec_to_hours(data: u64) -> u64 {
    (data % 86400) / 3600
}

/// time conversionsecord: sec to mins
/// This function converts seconds to minutes.
/// It takes a `u64` value representing the number of seconds and returns a `u64` value representing the equivalent in minutes.
/// It uses a constant conversion factor of 1 minute = 60 seconds.
pub fn sec_to_mins(data: u64) -> u64 {
    (data % 3600) / 60
}

/// percentage calculation
/// This function calculates the percentage of a value.
/// It takes two `f64` values: `used` and `total`.
/// It returns the percentage as a `f64` value.
pub fn percentage_cal(used: f64, total: f64) -> f64 {
    (used / total) * 100.0
}

/// Unit Conversion: Converts millivolts (mV) to volts (V).
/// This function takes a `usize` value representing millivolts and returns a `f64` value representing volts.
/// It divides the input value by 1_000.0 to convert it to volts.
pub fn mv_to_volts(mv: usize) -> f64 {
    (mv as f64) / 1_000.0
}

/// Unit conversion: Convert milliampere hours (mAh) to microampere hours (μAh).
/// This function takes a `usize` value representing milliampere hours and returns a `f64` value representing microampere hours.
/// It multiplies the input value by 1_000.0 to convert it to microampere hours.
/// It uses the `as` keyword to convert the `usize` value to `f64`.
/// The function returns the result as a `f64` value.
pub fn mah_to_uah(mah: usize) -> f64 {
    (mah as f64) * 1_000.0
}

/// Unit Conversion: Converts milliamps (mA) to amperes (A).
/// This function takes a `usize` value representing milliamps and returns a `f64` value representing amperes.
/// It divides the input value by 1_000.0 to convert it to amperes.
/// It uses the `as` keyword to convert the `usize` value to `f64`.
/// The function returns the result as a `f64` value.
pub fn ma_to_a(ma: usize) -> f64 {
    (ma as f64) / 1_000.0
}

/// Unit Conversion: Converts micro ampere hours (μAh) to milli ampere hours (mAh).
/// This function takes a `usize` value representing micro ampere hours and returns a `f64` value representing milli ampere hours.
/// It divides the input value by 1_000.0 to convert it to milli ampere hours.
/// It uses the `as` keyword to convert the `usize` value to `f64`.
/// The function returns the result as a `f64` value.
pub fn uah_to_mah(uah: usize) -> f64 {
    (uah as f64) / 1_000.0
}

/// Unit conversion: Converts microamps (μA) to milliamps (mA).
/// This function takes a `usize` value representing microamps and returns a `f64` value representing milliamps.
/// It divides the input value by 1_000.0 to convert it to milliamps.
/// It uses the `as` keyword to convert the `usize` value to `f64`.
/// The function returns the result as a `f64` value.
pub fn ua_to_ma(ua: usize) -> f64 {
    (ua as f64) / 1_000.0
}

/// Unit Conversion: Converts microvolts (µV) to volts (V).
/// This function takes a `usize` value representing microvolts and returns a `f64` value representing volts.
/// It divides the input value by 1_000_000.0 to convert it to volts.
/// It uses the `as` keyword to convert the `usize` value to `f64`.
/// The function returns the result as a `f64` value.
pub fn uv_to_volts(uv: usize) -> f64 {
    (uv as f64) / 1_000_000.0
}

/// Unit conversion: Converts milliamps (mA) to microamps (μA).
/// This function takes a `usize` value representing milliamps and returns a `f64` value representing microamps.
/// It divides the input value by 1_000.0 to convert it to microamps.
pub fn ma_to_ua(ma: usize) -> f64 {
    (ma as f64) * 1_000.0
}

/// Unit conversion: Converts ns to sec
/// This function takes a `u64` value representing nanoseconds and returns a `u64` value representing seconds.
/// It divides the input value by 1_000_000 to convert it to seconds.
pub fn ns_to_sec(time:u64) -> u64 {
    time/ 1_000_000
}

/// data struct: usize to bool
/// This function converts a `usize` value to a `bool`.
/// It checks if the value is 0 or 1.
/// If the value is 0, it returns `false`. If the value is 1, it returns `true`.
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
/// This function converts a `bool` value to a `usize`.
/// It checks if the value is `false` or `true`.
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
/// This function formats a given time in seconds into days, hours, and minutes.
/// It takes a `u64` value representing the time in seconds and returns a tuple containing the days, hours, and minutes.
/// The function calculates the days by dividing the total seconds by 86400.
/// It calculates the hours by taking the remainder of the total seconds divided by 86400 and dividing it by 3600.
/// Finally, it calculates the minutes by taking the remainder of the total seconds divided by 3600 and dividing it by 60.
/// The function returns a tuple containing the days, hours, and minutes.
pub fn format_times(time: u64) -> (u64, u64, u64) {
    let days = sec_to_day(time);
    let hours = sec_to_hours(time);
    let minutes = sec_to_mins(time);

    (days, hours, minutes)
}

/// format : time(hour,minutes,seconds)
///  This function converts a given time in seconds into hours, minutes, and seconds.
/// It takes a `u64` value representing the time in seconds and returns a tuple containing the hours, minutes, and seconds.
/// The function calculates the hours by dividing the total seconds by 3600.
/// It calculates the minutes by taking the remainder of the total seconds divided by 3600 and dividing it by 60.
/// Finally, it calculates the seconds by taking the remainder of the total seconds divided by 60.
/// The function returns a tuple containing the hours, minutes, and seconds.
pub fn sec_to_playingtime(time:u64) -> (u64,u64,u64) {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;

    (hours,minutes,seconds)
}

/// format: message
/// This function formats a message based on the result of a computation.
/// It takes a `Result` type as input, which can either be `Ok` with an `Option<String>` or an `Err` with a `Box<dyn Error>`.
/// If the result is `Ok` with a `Some` value, it returns the value as a string.
pub fn format_msg(result: Result<Option<String>, Box<dyn Error>>) -> String {
    match result {
        Ok(Some(data)) => data,
        Ok(None) => "No Found".to_string(),
        Err(e) => format!("ERROR={}", e),
    }
}

/// regex a once data
///  This function extracts the first match of a regex pattern from a string.
/// It takes a string `input` and a regex pattern `pattern` as input.
/// It returns the captured group as a `String`. If no match is found, it returns "Unknown".
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
/// This function extracts all matches of a regex pattern from a string and returns them as a vector of strings.
/// The function takes a string `output` and a regex pattern `pattern` as input.
/// It returns a vector of strings containing the captured groups.
pub fn regex_extract_vec(output: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    let output: Vec<String> = re
        .captures_iter(output)
        .map(|cap| cap[1].trim().to_string())
        .collect();

    output
}
