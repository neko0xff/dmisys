use crate::file;

/// Read: Motherboard Vendor
/// This indicates the vendor of the motherboard.
/// The value is read from the `/sys/class/dmi/id/sys_vendor` file.
/// The value is returned as a `String`.
pub fn read_host_vendor() -> String {
    file::read_dmi_path("sys_vendor")
}

/// Read: Motherboard Product Name
/// This indicates the product name of the motherboard.
/// The value is read from the `/sys/class/dmi/id/product_name` file.
/// The value is returned as a `String`.
pub fn read_host_boardname() -> String {
    file::read_dmi_path("board_name")
}

/// Read: Motherboard Model
/// This indicates the model of the motherboard.
/// The value is read from the `/sys/class/dmi/id/product_version` file.
/// The value is returned as a `String`.
pub fn read_host_model() -> String {
    file::read_dmi_path("product_name")
}
