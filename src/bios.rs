use crate::file;

/// Read: BIOS release version
/// This indicates the version of the BIOS firmware.
/// The value is read from the `/sys/class/dmi/id/bios_version` file.
/// The value is returned as a `String`.
pub fn read_bios_release() -> String {
    file::read_dmi_path("bios_release")
}

/// Read: BIOS Vendor
/// This indicates the vendor of the BIOS firmware.
/// The value is read from the `/sys/class/dmi/id/bios_vendor` file.
/// The value is returned as a `String`.
pub fn read_bios_vendor() -> String {
    file::read_dmi_path("bios_vendor")
}

/// Read: BIOS release a date
/// This indicates the release date of the BIOS firmware.
/// The value is read from the `/sys/class/dmi/id/bios_date` file.
/// The value is returned as a `String`.
pub fn read_bios_date() -> String {
    file::read_dmi_path("bios_date")
}

/// Read: Motherboard BIOS Version
/// This indicates the version of the motherboard BIOS.
/// The value is read from the `/sys/class/dmi/id/bios_version` file.
/// The value is returned as a `String`.
pub fn read_bios_version() -> String {
    file::read_dmi_path("bios_version")
}
