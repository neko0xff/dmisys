use std::fs;

pub fn read_bios_release() -> String {
    let bios_release = fs::read_to_string("/sys/class/dmi/id/bios_release")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", bios_release.trim())
}

pub fn read_bios_vendor() -> String {
    let bios_vendor = fs::read_to_string("/sys/class/dmi/id/bios_vendor")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", bios_vendor.trim())
}

pub fn read_bios_date() -> String {
    let bios_date = fs::read_to_string("/sys/class/dmi/id/bios_date")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", bios_date.trim())
}

pub fn read_bios_version() -> String {
    let bios_version = fs::read_to_string("/sys/class/dmi/id/bios_version")
        .unwrap_or_else(|_| "Unknown".to_string());
    format!("{}", bios_version.trim())
}