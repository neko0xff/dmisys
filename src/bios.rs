use crate::file;

pub fn read_bios_release() -> String {
    let dmi = "/sys/class/dmi/id/bios_release";
    let output = file::return_pathdata(dmi);

    output
}

pub fn read_bios_vendor() -> String {
    let dmi = "/sys/class/dmi/id/bios_vendor";
    let output = file::return_pathdata(dmi);

    output
}

pub fn read_bios_date() -> String {
    let dmi = "/sys/class/dmi/id/bios_date";
    let output = file::return_pathdata(dmi);

    output
}

pub fn read_bios_version() -> String {
    let dmi = "/sys/class/dmi/id/bios_version";
    let output = file::return_pathdata(dmi);

    output
}