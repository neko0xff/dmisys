use regex::Regex;

pub fn extract_info(output: &str, pattern: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(output) {
        captures.get(1).map_or("Unknown", |m| m.as_str()).to_string()
    } else {
        "Unknown".to_string()
    }
}