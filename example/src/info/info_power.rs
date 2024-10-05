use crate::ds::ds_table::*;
use dmisys::*;
use colored::*;
use tabled::Table;

fn supply_info() {
    let adp_count = device::read_adp_counts();
    let bat_count = device::read_bat_counts();
    let mut power_supplies = Vec::new();
    let mut battery_list = Vec::new();
    let mut battery_meta = Vec::new();
    let mut battery_health = Vec::new();

    println!("\n{}", "Power Supply Information".green().bold());
    println!("{}", "=========================".green());
    println!("{}","Find Device:".blue().bold());
    println!("ADP : {}", adp_count);
    println!("BAT : {}", bat_count);

    // ADP List
    if adp_count > 0 {
        println!("\n{}","ADP List".blue().bold());
        for adp_number in 1..=adp_count {
            let num = adp_number as u8;
            let name = supply::read_adp_name(num);
            let type_ = supply::read_adp_type(num);
            let online = if supply::read_adp_online(num) { "Yes" } else { "No" };

            power_supplies.push(PowerSupplyInfo {
                number: adp_number.to_string(),
                name,
                type_,
                online: online.to_string(),
            });
        }

        if !power_supplies.is_empty() {
            println!("{}", Table::new(power_supplies).to_string());
        } else {
            println!("No power supply devices found.");
        }
    }

    // BAT Info
    if bat_count > 0 {
        println!("\n{}","BAT Info".blue().bold());
        for bat_number in 1..=bat_count {
            let num = bat_number as u8;
            let name = battery::read_bat_name(num);
            let type_ = battery::read_bat_type(num);
            let capacity = battery::read_bat_capacity(num);
            let capacity_lv = battery::read_bat_capacity_lv(num);
            let status = battery::read_bat_status(num);
            let model = battery::read_bat_model(num);
            let serialnum = battery::read_bat_serialnum(num);
            let manufacturer = battery::read_bat_manufacturer(num);
            let present_str = if battery::read_bat_present(num) { "Yes" } else { "No" };
            let technology = battery::read_bat_technology(num);

            battery_list.push(BatteryInfo {
                number: bat_number.to_string(),
                name,
                type_,
                capacity,
                capacity_lv,
                model,
                serialnum,
                status,
                manufacturer,
                present: present_str.to_string(),
                technology,
            });
        }

        if !battery_list.is_empty() {
            println!("{}", Table::new(battery_list).to_string());
        } else {
            println!("No Battery devices found.");
        }

        // BAT Electron Info
        println!("\n{}","BAT Electron Info".blue().bold());
        for bat_number in 1..=bat_count {
            let num = bat_number as u8;
            let name = battery::read_bat_name(num);
            let charge_full_design = battery::read_bat_charge_full_design(num);
            let charge_now = battery::read_bat_charge_now(num);
            let current_now = battery::read_bat_current_now(num);
            let cyclecount = battery::read_bat_cyclecount(num);
            let volt_min = battery::read_bat_volt_min(num);
            let volt_now = battery::read_bat_volt_now(num);

            battery_meta.push(BatteryElectronMeta {
                number: bat_number.to_string(),
                name,
                charge_now,
                charge_full_design,
                cyclecount,
                volt_min,
                volt_now,
                current_now,
            });
        }

        if !battery_meta.is_empty() {
            println!("{}", Table::new(battery_meta).to_string());
        } else {
            println!("No Battery electron data found.");
        }

        // BAT Health Info
        println!("\n{}","BAT Health Info".blue().bold());
        for bat_number in 1..=bat_count {
            let num = bat_number as u8;
            let name = battery::read_bat_name(num);
            let health_per = battery::read_bat_health(num);
            let life_time_use = battery::read_bat_timelife(num);

            battery_health.push(BatteryHealthInfo {
                number: bat_number.to_string(),
                name,
                percentage: format!("{:.2}",health_per).to_string(),
                life_time: format!("{:.2}",life_time_use).to_string(),
            });
        }

        if !battery_health.is_empty() {
            println!("{}", Table::new(battery_health).to_string());
        } else {
            println!("No Battery health data found.");
        }
    }
}

pub fn output_msg() {
    println!("\n{}", "Power Information".green().bold());
    println!("{}", "==================".green());
    println!("{:<25} {} ms", "Autosuspend Delay:".blue().bold(), power::read_autosuspend_delay_ms());
    println!("{:<25} {}", "Control:".blue().bold(), power::read_control());
    println!("{:<25} {}", "Runtime Status:".blue().bold(), power::read_runtime_status());
    println!("{:<25} {}", "Runtime Active Time:".blue().bold(), power::read_runtime_active_time());
    println!("{:<25} {}", "Runtime Suspended Time:".blue().bold(), power::read_runtime_suspended_time());

    if !file::check_directory_null("/sys/class/power_supply/") {
        supply_info();
    }
}
