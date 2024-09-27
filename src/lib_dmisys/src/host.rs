use std::fs;

pub fn read_host_vendor() -> String {
    let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", vendor.trim())
}
pub fn read_host_boardname() -> String {
    let boardname =  fs::read_to_string("/sys/class/dmi/id/board_name")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", boardname.trim())
}
pub fn read_host_model() -> String {
    let model = fs::read_to_string("/sys/class/dmi/id/product_name")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", model.trim())
}