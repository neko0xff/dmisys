use std::process::Command;
use crate::cv;

/*讀取PCI上的裝置*/
pub fn run_cmd_devicepci() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci")
        .output()
        .expect("Failed to execute command");
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn read_device_info() -> String {
    let output = run_cmd_devicepci();

    output
}

pub fn read_device_gpu() -> Vec<String> {
    let input = read_device_info();
    let regex_pattern =  r"VGA compatible controller:\s*(.+)";
    let output = cv::regex_extract_vec(&input, regex_pattern);

    output
}