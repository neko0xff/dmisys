use crate::{
    cv,
    file
};

/// Read battery information from /sys/class/power_supply/BAT\[number]\/uevent
pub fn read_bat_info(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);

    file::read_config_info(&file)
}

/// Read battery device type
pub fn read_bat_devtype(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "DEVTYPE=";

    file::read_config_var_string(&file, find)
}

/// Read battery device name
pub fn read_bat_name(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_NAME=";

    file::read_config_var_string(&file, find)
}

/// Read battery type
pub fn read_bat_type(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_TYPE=";

    file::read_config_var_string(&file, find)
}

/// Read battery status(charging, discharging, full, etc.)
pub fn read_bat_status(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_STATUS=";

    file::read_config_var_string(&file, find)
}

/// Read battery present status
pub fn read_bat_present(bat_number: u8) -> bool {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_PRESENT=";

    file::read_config_var_bool(&file, find)
}

/// Read battery use technology
pub fn read_bat_technology(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_TECHNOLOGY=";

    file::read_config_var_string(&file, find)
}

/// Read battery used charge cycle count
pub fn read_bat_cyclecount(bat_number: u8) -> usize {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CYCLE_COUNT=";

    file::read_config_var_usize(&file, find)
}

/// check the battery voltage min design
pub fn read_bat_volt_min(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_VOLTAGE_MIN_DESIGN=";
    let num = file::read_config_var_usize(&file, find);

    cv::uv_to_volts(num)
}

/// check the battery voltage now
pub fn read_bat_volt_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_VOLTAGE_NOW=";
    let num = file::read_config_var_usize(&file, find);

    cv::uv_to_volts(num)
}

/// check the battery charge now
pub fn read_bat_charge_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CHARGE_NOW=";
    let num = file::read_config_var_usize(&file, find);

    cv::uah_to_mah(num)
}

/// check the battery charge full design
pub fn read_bat_charge_full_design(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CHARGE_FULL_DESIGN=";
    let num = file::read_config_var_usize(&file, find);

    cv::uah_to_mah(num)
}

/// check the battery capacity
pub fn read_bat_capacity(bat_number: u8) -> usize {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CAPACITY=";

    file::read_config_var_usize(&file, find)
}

/// check the battery capacity level in percent
pub fn read_bat_capacity_lv(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CAPACITY_LEVEL=";

    file::read_config_var_string(&file, find)
}

/// check the battery current in ampere
pub fn read_bat_current_now(bat_number: u8) -> f64 {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_CURRENT_NOW=";
    let num = file::read_config_var_usize(&file, find);

    cv::ma_to_a(num)
}

/// check the battery model name
pub fn read_bat_model(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_MODEL_NAME=";

    file::read_config_var_string(&file, find)
}

/// check the battery manufacturer name
pub fn read_bat_manufacturer(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_MANUFACTURER=";

    file::read_config_var_string(&file, find)
}

/// check the battery serial number
pub fn read_bat_serialnum(bat_number: u8) -> String {
    let read_bat = format!("BAT{}", bat_number);
    let file = format!("/sys/class/power_supply/{}/uevent", read_bat);
    let find = "POWER_SUPPLY_SERIAL_NUMBER=";

    file::read_config_var_string(&file, find)
}

/// check remaining time of the battery life
pub fn read_bat_timelife(bat_number: u8) -> f64 {
    let charge_now_uah = read_bat_charge_now(bat_number);
    let current_now_ua = read_bat_current_now(bat_number);
    let charge_now_mah = cv::uah_to_mah(charge_now_uah as usize);
    let current_now_ma = cv::ua_to_ma(current_now_ua as usize);
    let output: f64;

    if current_now_ma > 0.0 {
        output = charge_now_mah / current_now_ma;
    } else {
        output = 0.0;
    }

    output
}

/// check the battery health percentage
pub fn read_bat_health(bat_number: u8) -> f64 {
    let charge_full_design_uah = read_bat_charge_full_design(bat_number);
    let charge_full_uah = read_bat_charge_now(bat_number);
    let charge_full_design_mah = cv::uah_to_mah(charge_full_design_uah as usize);
    let charge_full_mah = cv::uah_to_mah(charge_full_uah as usize);
    let output: f64;

    if charge_full_design_mah > 0.0 {
        output = cv::percentage_cal(charge_full_mah, charge_full_design_mah);
    } else {
        output = 0.0;
    }

    output
}
