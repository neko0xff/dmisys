use crate::file;

/// BIOS release version
pub fn read_bios_release() -> String {
    let dmi = "/sys/class/dmi/id/bios_release";

    file::return_pathdata(dmi)
}

/// BIOS Vendor
pub fn read_bios_vendor() -> String {
    let dmi = "/sys/class/dmi/id/bios_vendor";

    file::return_pathdata(dmi)
}

/// BIOS release a date
pub fn read_bios_date() -> String {
    let dmi = "/sys/class/dmi/id/bios_date";

    file::return_pathdata(dmi)
}

/// Motherboard BIOS Version
pub fn read_bios_version() -> String {
    let dmi = "/sys/class/dmi/id/bios_version";

    file::return_pathdata(dmi)
}
