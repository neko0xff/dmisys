use std::fs;

#[warn(dead_code)]
pub fn read_release() -> String {
    let file = "/etc/os-release";
    if let Ok(contents) = fs::read_to_string(file) {
        return contents;
    }
    "OS information not available".to_string()
}

pub fn read_distro_name() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("NAME=") {
                return line.trim_start_matches("NAME=").replace("\"", "").to_string();
            }
        }
    }
    "Unknown Distro".to_string()
}