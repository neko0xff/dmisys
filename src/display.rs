use crate::cv;
use std::process::Command;

/// Read the output of xrandr command and parse the resolution
fn read_cmd_xrandr() -> Result<String, std::io::Error> {
    let output = Command::new("xrandr").arg("--current").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Read the output of xdpyinfo command and parse the resolution
fn read_cmd_xdpyinfo() -> Result<String, std::io::Error> {
    let output = Command::new("xdpyinfo").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_output_xrandr(input: &str) -> Option<String> {
    let regex_pattern = r"current (\d+) x (\d+)";
    let output = cv::regex_extract(input, regex_pattern);

    Some(output)
}

fn parse_output_xdpyinfo(input: &str) -> Option<String> {
    let regex_pattern = r"dimensions:\s+(\d+x\d+)";
    let output = cv::regex_extract(input, regex_pattern);

    Some(output)
}

fn get_info_resolution() -> Option<String> {
    if let Ok(xrandr_output) = read_cmd_xrandr() {
        if let Some(resolution) = parse_output_xrandr(&xrandr_output) {
            return Some(resolution);
        }
    }

    if let Ok(xdpyinfo_output) = read_cmd_xdpyinfo() {
        if let Some(resolution) = parse_output_xdpyinfo(&xdpyinfo_output) {
            return Some(resolution);
        }
    }

    None
}

fn read_cmd_xserver() -> Result<String, std::io::Error> {
    let output = Command::new("X")
        .arg("-version")
        .output()?;

    let result = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(result)
}


/// Display Resolution
pub fn read_display_resolution() -> String {
    let output;

    match get_info_resolution() {
        Some(resolution) => output = resolution.to_string(),
        None => output = "Unknow".to_string(),
    }

    output.to_string()
}

///  Xorg Server: Verion
pub fn read_xserver_ver() -> Option<String> {
    let output = read_cmd_xserver().ok()?; 
    let regex_pattern = r"X.Org X Server (\d+\.\d+\.\d+\.\d+)";
    
    Some(cv::regex_extract(&output, regex_pattern).into())
}

