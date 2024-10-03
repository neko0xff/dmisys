use crate::{
    file,
    device
};

pub fn read_bat_counts() -> usize {
    let output = device::find_devices_counts("BAT").len();

    output
}

pub fn read_bat_info(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let output = file::read_config_info(&file);

    output
}

pub fn read_bat_devtype(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "DEVTYPE=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_name(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_NAME=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_type(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_TYPE=";
    let output = file::read_config_var_string(&file, find);

    output
}