use crate::file;

/// Read ADP information from /sys/class/power_supply/ADP\[number]\/uevent
pub fn read_adp_info(adp_number: u8) -> String {
    let read_adp = format!("ADP{}", adp_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_adp);
    let output = file::read_config_info(&file);

    output
}

/// Read ADP device type
pub fn read_adp_devtype(adp_number: u8) -> String {
    let read_adp = format!("ADP{}", adp_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_adp);
    let find = "DEVTYPE=";
    let output = file::read_config_var_string(&file, find);

    output
}

/// Read ADP device name
pub fn read_adp_name(adp_number: u8) -> String {
    let read_adp = format!("ADP{}", adp_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_adp);
    let find = "POWER_SUPPLY_NAME=";
    let output = file::read_config_var_string(&file, find);

    output
}

/// Read ADP device type
pub fn read_adp_type(adp_number: u8) -> String {
    let read_adp = format!("ADP{}", adp_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_adp);
    let find = "POWER_SUPPLY_TYPE=";
    let output = file::read_config_var_string(&file, find);

    output
}

/// check the ADP online status(online, offline)
pub fn read_adp_online(adp_number: u8) -> bool {
    let read_adp = format!("ADP{}", adp_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_adp);
    let find = "POWER_SUPPLY_ONLINE=";
    let output = file::read_config_var_bool(&file, find);

    output
}