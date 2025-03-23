use crate::file;

/// BIOS release version
pub fn read_bios_release() -> String {
    file::read_dmi_path("bios_release")
}

/// BIOS Vendor
pub fn read_bios_vendor() -> String {
    file::read_dmi_path("bios_vendor")
}

/// BIOS release a date
pub fn read_bios_date() -> String {
    file::read_dmi_path("bios_date")
}

/// Motherboard BIOS Version
pub fn read_bios_version() -> String {
    file::read_dmi_path("bios_version")
}
