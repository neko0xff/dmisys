use crate::file;

pub fn read_host_vendor() -> String {
    let dmi = "/sys/class/dmi/id/sys_vendor";
    let output = file::return_pathdata(dmi);

    output
}
pub fn read_host_boardname() -> String {
    let dmi = "/sys/class/dmi/id/board_name";
    let output = file::return_pathdata(dmi);

    output
}
pub fn read_host_model() -> String {
    let dmi = "/sys/class/dmi/id/product_name";
    let output = file::return_pathdata(dmi);

    output
}