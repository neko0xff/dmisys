use crate::{
    cv,
    file
};

/// Read BAT information from /sys/class/power_supply/ADP[number]/uevent
fn read_bat_path(bat_number: u8) -> String {
    file::read_power_path("BAT",bat_number)
}

/// Read battery information from /sys/class/power_supply/BAT\[number]\/uevent
pub fn read_bat_info(bat_number: u8) -> String {
    let path = read_bat_path(bat_number);

    file::read_config_info(&path)
}

/// Extract a specific value from ADP information
/// This function reads the battery information from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// It extracts the value associated with the specified key.
/// The value is returned as a `String`.
fn read_bat_value(bat_number: u8, key: &str) -> String {
    let path = read_bat_path(bat_number);
    file::read_config_var_string(&path, key)
}

/// Read: battery device type
/// This indicates the type of power supply device.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_devtype(bat_number: u8) -> String {
    read_bat_value(bat_number, "DEVTYPE=")
}

/// Read: battery device name
///  This indicates the name of the power supply device.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_name(bat_number: u8) -> String {
    read_bat_value(bat_number, "POWER_SUPPLY_NAME=")
}

/// Read: battery type
/// This indicates the type of power supply device.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_type(bat_number: u8) -> String {
    read_bat_value(bat_number, "POWER_SUPPLY_TYPE=")
}

/// Read: battery status(charging, discharging, full, etc.)
/// This indicates the current status of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_status(bat_number: u8) -> String {
    read_bat_value(bat_number, "POWER_SUPPLY_STATUS=")
}

/// Read: battery present status
/// This indicates whether the battery is present or not.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `bool`.
/// Return:  true == present, false == not present
pub fn read_bat_present(bat_number: u8) -> bool {
    let path = read_bat_path(bat_number);

    file::read_config_var_bool(&path, "POWER_SUPPLY_PRESENT=")
}

/// Read: battery use technology
/// This indicates the technology used in the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_technology(bat_number: u8) -> String {
    read_bat_value(bat_number, "POWER_SUPPLY_TECHNOLOGY=")
}

/// Read: battery used charge cycle count
/// This indicates the number of charge cycles the battery has gone through.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `usize`.
/// The value is in microampere-hours (uAh).
pub fn read_bat_cyclecount(bat_number: u8) -> usize {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CYCLE_COUNT=";

    file::read_config_var_usize(&path, find)
}

/// Read: the battery voltage min design
/// This indicates the minimum voltage of the battery design.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `f64` (double precision floating point number).
/// The value is in microvolts.
pub fn read_bat_volt_min(bat_number: u8) -> f64 {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_VOLTAGE_MIN_DESIGN=";
    let num = file::read_config_var_usize(&path, find);

    cv::uv_to_volts(num)
}

/// Read: the battery voltage now
/// The value is in percent.
/// This indicates the current voltage of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `f64` (double precision floating point number).
pub fn read_bat_volt_now(bat_number: u8) -> f64 {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_VOLTAGE_NOW=";
    let num = file::read_config_var_usize(&path, find);

    cv::uv_to_volts(num)
}

/// Read:  the battery charge now
pub fn read_bat_charge_now(bat_number: u8) -> f64 {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CHARGE_NOW=";
    let num = file::read_config_var_usize(&path, find);

    cv::uah_to_mah(num)
}

/// Read: the battery charge full design
/// This indicates the full design charge of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `f64` (double precision floating point number).
/// The value is in microampere-hours (uAh).
pub fn read_bat_charge_full_design(bat_number: u8) -> f64 {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CHARGE_FULL_DESIGN=";
    let num = file::read_config_var_usize(&path, find);

    cv::uah_to_mah(num)
}

/// Read:  the battery capacity
/// This indicates the battery's charge level.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `usize`.
pub fn read_bat_capacity(bat_number: u8) -> usize {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CAPACITY=";

    file::read_config_var_usize(&path, find)
}

/// Read:  the battery capacity level in percent
/// This indicates the battery's charge level.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_capacity_lv(bat_number: u8) -> String {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CAPACITY_LEVEL=";

    file::read_config_var_string(&path, find)
}

/// Read:  the battery current in ampere
/// This indicates the current being drawn from the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `f64` (double precision floating point number).
pub fn read_bat_current_now(bat_number: u8) -> f64 {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_CURRENT_NOW=";
    let num = file::read_config_var_usize(&path, find);

    cv::ma_to_a(num)
}

/// Read:  the battery model name
/// This indicates the model name of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_model(bat_number: u8) -> String {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_MODEL_NAME=";

    file::read_config_var_string(&path, find)
}

/// Read:  the battery manufacturer name
///  This indicates the manufacturer of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_manufacturer(bat_number: u8) -> String {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_MANUFACTURER=";

    file::read_config_var_string(&path, find)
}

/// Read:  the battery serial number
/// This indicates the serial number of the battery.
/// The value is read from the `/sys/class/power_supply/BAT[number]/uevent` file.
/// The value is returned as a `String`.
pub fn read_bat_serialnum(bat_number: u8) -> String {
    let path = read_bat_path(bat_number);
    let find = "POWER_SUPPLY_SERIAL_NUMBER=";

    file::read_config_var_string(&path, find)
}

/// Read:  remaining time of the battery life
/// This function calculates the remaining time of the battery life based on the current charge and the current draw.
/// It retrieves the current charge and the current draw from the battery information.
/// It converts the values from microampere-hours (uAh) to milliampere-hours (mAh).
/// It then calculates the remaining time by dividing the current charge by the current draw.
/// If the current draw is greater than zero, it returns the calculated time.
/// Otherwise, it returns 0.0.
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

/// Read:  the battery health percentage
/// This function calculates the battery health percentage based on the current charge and the design capacity.
/// It retrieves the full design charge and the current charge from the battery information.
/// It converts the values from microampere-hours (uAh) to milliampere-hours (mAh).
/// It then calculates the health percentage by dividing the current charge by the design charge.
/// If the design charge is greater than zero, it returns the calculated percentage.
/// Otherwise, it returns 0.0.
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
