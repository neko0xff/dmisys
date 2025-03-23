use crate::file;

/// Motherboard Vendor
pub fn read_host_vendor() -> String {
    file::read_dmi_path("sys_vendor")
}

/// Motherboard Product Name
pub fn read_host_boardname() -> String {
    file::read_dmi_path("board_name")
}

/// Motherboard Model
pub fn read_host_model() -> String {
    file::read_dmi_path("product_name")
}
