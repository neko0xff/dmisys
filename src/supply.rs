use crate::file;

/// Read ADP information from /sys/class/power_supply/ADP[number]/uevent
fn read_adp_info(adp_number: u8) -> String {
    file::read_power_path("ADP",adp_number)
}

/// Extract a specific value from ADP information
fn read_adp_value(adp_number: u8, key: &str) -> String {
    let path = read_adp_info(adp_number);
    file::read_config_var_string(&path, key)
}

/// ADP: device type
pub fn read_adp_devtype(adp_number: u8) -> String {
    read_adp_value(adp_number, "DEVTYPE=")
}

/// ADP: device name
pub fn read_adp_name(adp_number: u8) -> String {
    read_adp_value(adp_number, "POWER_SUPPLY_NAME=")
}

/// ADP: power supply type
pub fn read_adp_type(adp_number: u8) -> String {
    read_adp_value(adp_number, "POWER_SUPPLY_TYPE=")
}

/// ADP: device status
/// Return:  true == online, false == offline
pub fn read_adp_online(adp_number: u8) -> bool {
    let path = read_adp_info(adp_number);

    file::read_config_var_bool(&path, "POWER_SUPPLY_ONLINE=")
}
