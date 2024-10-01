use regex::Regex;

/*單位轉換*/
pub fn bytes_to_gb(bytes: u64) -> f64 {
    let cv_gb = f64::powf(2 as f64,30 as f64);
    let output = (bytes as f64) / cv_gb;

    output
}

pub fn bytes_to_mb(data:u64) -> u64 {
    let cv_mb = u64::pow(2,20);
    let output = data / cv_mb;
    
    output 
}

pub fn sec_to_day(data:u64) -> u64{
    let output =  data/86400;

    output
}

pub fn sec_to_hours(data:u64) -> u64{
    let output =  (data % 86400)/3600;

    output
}

pub fn sec_to_mins(data:u64) -> u64{
    let output =  (data % 3600)/60;

    output
}

pub fn sectors_to_gb(data:u64) -> f64 {
    let output = (data * 512) as f64 / 1_073_741_824.0;

    output 
}

pub fn format_times(time: u64) -> (u64,u64,u64) {
    let days = sec_to_day(time);
    let hours = sec_to_hours(time);
    let minutes = sec_to_mins(time);

    (days, hours, minutes)
}

/*正規表示法*/
pub fn regex_extract(output: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    
    if let Some(captures) = re.captures(output) {
        captures.get(1).map_or("Unknown", |m| m.as_str()).to_string()
    } else {
        "Unknown".to_string()
    }
}