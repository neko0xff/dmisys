use crate::file;

pub fn read_main_info() -> String {
    let file = "/sys/class/power_supply/ADP1/uevent";
    let output = file::read_config_info(file);

    output
}

pub fn read_main_devtype() -> String {
    let file = "/sys/class/power_supply/ADP1/uevent";
    let find = "DEVTYPE=";
    let output = file::read_config_var_string(file, find);

    output
}

pub fn read_main_name() -> String {
    let file = "/sys/class/power_supply/ADP1/uevent";
    let find = "POWER_SUPPLY_NAME=";
    let output = file::read_config_var_string(file, find);

    output
}

pub fn read_main_type() -> String {
    let file = "/sys/class/power_supply/ADP1/uevent";
    let find = "POWER_SUPPLY_TYPE=";
    let output = file::read_config_var_string(file, find);

    output
}

pub fn read_main_online() -> bool {
    let file = "/sys/class/power_supply/ADP1/uevent";
    let find = "POWER_SUPPLY_ONLINE=";
    let output = file::read_config_var_bool(file, find);

    output
}