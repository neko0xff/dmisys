use regex::Regex;
use std::error::Error;

/// 容量: GB
pub fn bytes_to_gb(bytes: u64) -> f64 {
    let cv_gb = f64::powf(2 as f64,30 as f64);
    let output = (bytes as f64) / cv_gb;

    output
}

/// 容量: MB
pub fn bytes_to_mb(data:u64) -> u64 {
    let cv_mb = u64::pow(2,20);
    let output = data / cv_mb;
    
    output 
}

/// 容量: 磁區計算
pub fn sectors_to_gb(data:u64) -> f64 {
    let output = (data * 512) as f64 / 1_073_741_824.0;

    output 
}

/// 單位轉換: 時間(秒轉日)
pub fn sec_to_day(data:u64) -> u64{
    let output =  data/86400;

    output
}

/// 單位轉換: 時間(秒轉時)
pub fn sec_to_hours(data:u64) -> u64{
    let output =  (data % 86400)/3600;

    output
}

/// 單位轉換: 時間(秒轉分鐘)
pub fn sec_to_mins(data:u64) -> u64{
    let output =  (data % 3600)/60;

    output
}

/// 百分比計算
pub fn percentage_cal(used:f64,total:f64) -> f64{
    let output = (used / total) as f64 * 100.0;

    output
}

/// 單位轉換: 將毫伏 (mV) 轉換為伏特 (V)
pub fn mv_to_volts(mv: usize) -> f64 {
    let output = (mv as f64) / 1_000.0;
    output
}

/// 單位轉換: 將毫安小時 (mAh) 轉換為微安小時 (μAh)
pub fn mah_to_uah(mah: usize) -> f64 {
    let output = (mah as f64) * 1_000.0;
    output
}

/// 單位轉換: 將毫安 (mA) 轉換為安培 (A)
pub fn ma_to_a(ma: usize) -> f64 {
    let output = (ma as f64) / 1_000.0;
    output
}

/// 單位轉換: 將微安小時 (μAh) 轉換為毫安小時 (mAh)
pub fn uah_to_mah(uah: usize) -> f64 {
    let output = (uah as f64) / 1_000.0;
    output
}

/// 單位轉換: 將微安 (μA) 轉換為毫安 (mA)
pub fn ua_to_ma(ua: usize) -> f64 {
    let output = (ua as f64) / 1_000.0;
    output
}

/// 單位轉換: 將微伏 (μV) 轉換為伏特 (V)
pub fn uv_to_volts(uv: usize) -> f64 {
    let output = (uv as f64) / 1_000_000.0;
    output
}

/// 單位轉換: 將毫安 (mA) 轉換為微安 (μA)
pub fn ma_to_ua(ma: usize) -> f64 {
    let output = (ma as f64) * 1_000.0;
    output
}

/// 型別轉換: 無符號整數轉布林
pub fn usize_to_bool(n: usize) -> bool{
    let mut out = false;
    
    if n == 0 {
        out = false;
    }else if n == 1 {
        out = true;
    }

    out 
}

///  型別轉換: 布林轉無符號整數
pub fn bool_to_usize(n: bool) -> usize{
    let mut out:usize = 0;

    if n == false {
        out = 0;
    }else if  n== true {
        out = 1;
    }

    out
}

/// 格式化輸出: 時間(日,小時,分鐘)
pub fn format_times(time: u64) -> (u64,u64,u64) {
    let days = sec_to_day(time);
    let hours = sec_to_hours(time);
    let minutes = sec_to_mins(time);

    (days, hours, minutes)
}

/// 格式化輸出: 回傳訊息處理
pub fn format_msg(result: Result<Option<String>, Box<dyn Error>>) -> String {
    match result {
        Ok(Some(data)) => data,
        Ok(None) => "No Found".to_string(),
        Err(e) => format!("ERROR={}", e),
    }
}

/// 正規表示法: 單筆
pub fn regex_extract(input: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    
    if let Some(captures) = re.captures(input) {
        captures.get(1).map_or("Unknown", |m| m.as_str()).to_string()
    } else {
        "Unknown".to_string()
    }
}

/// 正規表示法:多筆
pub fn regex_extract_vec(output: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    let output: Vec<String> = re.captures_iter(&output)
        .map(|cap| cap[1].trim().to_string())
        .collect();

    output
}
