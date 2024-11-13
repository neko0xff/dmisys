use crate::file;

/// Motherboard Vendor
pub fn read_host_vendor() -> String {
    let dmi = "/sys/class/dmi/id/sys_vendor";

    file::return_pathdata(dmi)
}

/// Motherboard Product Name
pub fn read_host_boardname() -> String {
    let dmi = "/sys/class/dmi/id/board_name";

    file::return_pathdata(dmi)
}

/// Motherboard Model
pub fn read_host_model() -> String {
    let dmi = "/sys/class/dmi/id/product_name";

    file::return_pathdata(dmi)
}
