use crate::{cv, file};

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

pub fn read_bat_status(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_STATUS=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_present(bat_number: u8) -> bool {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_PRESENT=";
    let output = file::read_config_var_bool(&file, find);

    output
}

pub fn read_bat_technology(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_TECHNOLOGY=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_cyclecount(bat_number: u8) -> usize {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CYCLE_COUNT=";
    let output = file::read_config_var_usize(&file, find);

    output
}

pub fn read_bat_volt_min(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_VOLTAGE_MIN_DESIGN=";
    let num = file::read_config_var_usize(&file, find) ;
    let output = cv::mv_to_volts(num);

    output
}

pub fn read_bat_volt_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_VOLTAGE_NOW=";
    let num = file::read_config_var_usize(&file, find) ;
    let output = cv::mv_to_volts(num);

    output
}

pub fn read_bat_charge_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CHARGE_NOW=";
    let num = file::read_config_var_usize(&file, find);
    let output = cv::mah_to_uah(num);

    output
}

pub fn read_bat_charge_full_design(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CHARGE_FULL_DESIGN=";
    let num = file::read_config_var_usize(&file, find);
    let output = cv::mah_to_uah(num);

    output
}

pub fn read_bat_capacity(bat_number: u8) -> usize {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CAPACITY=";
    let output = file::read_config_var_usize(&file, find);

    output
}

pub fn read_bat_capacity_lv(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CAPACITY_LEVEL=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_current_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CURRENT_NOW=";
    let num = file::read_config_var_usize(&file, find);
    let output = cv::ma_to_a(num);

    output
}

pub fn read_bat_model(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_MODEL_NAME=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_manufacturer(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_MANUFACTURER=";
    let output = file::read_config_var_string(&file, find);

    output
}

pub fn read_bat_serialnum(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_SERIAL_NUMBER=";
    let output = file::read_config_var_string(&file, find);

    output
}


