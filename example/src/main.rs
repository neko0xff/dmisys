mod ds;
use dmisys::*;
use colored::*;
use tabled::Table;
use crate::ds::ds_table::*;

fn main() {
    let memory_read = memory::Info::new();
    let gpus = device::read_device_gpu();
    let disks_drive = disk::read_disk_all_vec();
    let sector_data = disk::read_disk_sectorspace_vec();
    let disk_list = disk::read_disks_physicaldrive_list();
    let (days_up, hours_up, minutes_up) = systime::read_systime_up();
    let (days_unix, hours_unix, minutes_unix) = systime::read_systime_boot();
    let (io_write, io_read) = os::read_io_speed();
    let local_ipv64 = network::get_local_ipv64();
    let network_speed = network::get_speed();
    let mac_address = network::get_macaddress();

    println!("{}", "System Information".green().bold());
    println!("{}", "=================".green());
    println!("{:<15} {}", "OS:".blue().bold(), os::read_osname());
    println!("{:<15} {}", "Distro:".blue().bold(), os::read_distro_name());
    println!("{:<15} {}", "Host Name:".blue().bold(), os::read_hostname());
    println!("{:<15} {}", "Kernel:".blue().bold(), os::read_kernel());
    println!("{:<15} {}", "Vendor:".blue().bold(), host::read_host_vendor());
    println!("{:<15} {}", "Board:".blue().bold(), host::read_host_boardname());
    println!("{:<15} {}", "Model:".blue().bold(), host::read_host_model());
    println!("{:<15} {} days, {} hours, {} minutes", "Uptime:".blue().bold(), days_up, hours_up, minutes_up);
    println!("{:<15} {} days, {} hours, {} minutes", "Unix time:".blue().bold(), days_unix, hours_unix, minutes_unix);
    println!("{:<15} Write = {} MB / Read = {} MB", "IO:".blue().bold(), io_write, io_read);

    println!("\n{}", "Network".green().bold());
    println!("{}", "====================".green());
    println!("{}", "Public Address:".blue().bold());
    println!("{:<15} {}", "IPv4:".yellow(), network::get_public_ipv4_address());
    println!("{:<15} {}", "IPv6:".yellow(), network::get_public_ipv6_address());

    println!("\n{}", "Local Address:".blue().bold());
    if local_ipv64.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let network_info: Vec<NetworkInfo> = local_ipv64.iter().map(|(interface, ipv4, ipv6)| {
            NetworkInfo {
                interface: interface.to_string(),
                ipv4: ipv4.to_string(),
                ipv6: ipv6.to_string(),
            }
        }).collect();
        println!("{}", Table::new(network_info).to_string());
    }

    println!("\n{}", "Network Speed:".blue().bold());
    if network_speed.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let speed_info: Vec<NetworkSpeed> = network_speed.iter().map(|(interface, tx, rx)| {
            NetworkSpeed {
                interface: interface.to_string(),
                tx_speed: format!("{} Mb", tx),
                rx_speed: format!("{} Mb", rx),
            }
        }).collect();
        println!("{}", Table::new(speed_info).to_string());
    }

    println!("\n{}", "Mac Address:".blue().bold());
    if mac_address.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let mac_info: Vec<MacAddress> = mac_address.iter().map(|(interface, mac)| {
            MacAddress {
                interface: interface.to_string(),
                mac: mac.to_string(),
            }
        }).collect();
        println!("{}", Table::new(mac_info).to_string());
    }

    println!("\n{}", "CPU Information".green().bold());
    println!("{}", "================".green());
    println!("{:<20} {:?}", "CPU Model:".blue().bold(), cpu::read_cpu_model());
    println!("{:<20} {:.4} GHz", "CPU Frequency:".blue().bold(), cpu::get_cpu_frequency());
    println!("{:<20} {:?}", "CPU Core:".blue().bold(), cpu::read_cpu_cores());
    println!("{:<20} {:?}", "CPU Threads:".blue().bold(), cpu::read_cpu_threads());
    println!("{:<20} {}", "CPU Arch:".blue().bold(), cpu::read_cpu_arch());
    println!("{:<20} {}%", "CPU Load Avg:".blue().bold(), cpu::get_cpu_loading().to_string());

    println!("\n{}", "GPU Information".green().bold());
    println!("{}", "================".green());
    if gpus.is_empty() {
        println!("No GPUs found");
    } else {
        for (index, gpu) in gpus.iter().enumerate() {
            println!("{:<4} {}", format!("GPU {}:", index + 1).blue().bold(), gpu);
        }
    }

    println!("\n{}", "BIOS Information".green().bold());
    println!("{}", "=================".green());
    println!("{:<15} {}", "Vendor:".blue().bold(), bios::read_bios_vendor());
    println!("{:<15} {}", "Release:".blue().bold(), bios::read_bios_release());
    println!("{:<15} {}", "Version:".blue().bold(), bios::read_bios_version());
    println!("{:<15} {}", "Date:".blue().bold(), bios::read_bios_date());

    println!("\n{}", "Memory Information".green().bold());
    println!("{}", "===================".green());
    println!("{:<15} {}MB ({:.2}%)", "Used:".blue().bold(), memory_read.used_memory(), memory_read.used_memory_percent());
    println!("{:<15} {}MB ({:.2}%)", "Free:".blue().bold(), memory_read.free_memory(), memory_read.free_memory_percent());
    println!("{:<15} {}MB ({:.2}%)", "Available:".blue().bold(), memory_read.available_memory(), memory_read.available_memory_percent());
    println!("{:<15} {}MB", "Total:".blue().bold(), memory_read.total_memory());

    println!("\n{}", "Swap Information".green().bold());
    println!("{}", "=================".green());
    println!("{:<15} {}MB ({:.2}%)", "Used:".blue().bold(), memory_read.used_swap(), memory_read.used_swap_percent());
    println!("{:<15} {}MB ({:.2}%)", "Free:".blue().bold(), memory_read.free_swap(), memory_read.free_swap_percent());
    println!("{:<15} {}MB", "Total:".blue().bold(), memory_read.total_swap());

    println!("\n{}", "Power Information".green().bold());
    println!("{}", "==================".green());
    println!("{:<25} {} ms", "Autosuspend Delay:".blue().bold(), power::read_autosuspend_delay_ms());
    println!("{:<25} {}", "Control:".blue().bold(), power::read_control());
    println!("{:<25} {}", "Runtime Status:".blue().bold(), power::read_runtime_status());
    println!("{:<25} {}", "Runtime Active Time:".blue().bold(), power::read_runtime_active_time());
    println!("{:<25} {}", "Runtime Suspended Time:".blue().bold(), power::read_runtime_suspended_time());

    
    if file::check_directory_null("/sys/class/power_supply/") == false {
        let adp_count = device::read_adp_counts();
        let bat_count = device::read_bat_counts();
        let mut power_supplies = Vec::new();
        let mut battery_list = Vec::new();
        let mut battery_meta = Vec::new();

        println!("\n{}", "Power Supply Information".green().bold());
        println!("{}", "=========================".green());
        println!("{}","Find Device:".blue().bold());
        println!("ADP : {}",adp_count);
        println!("BAT : {}",bat_count);

        println!("\n{}","ADP List".blue().bold());
        for adp_number in 1..adp_count+1 {
            let num = adp_number as u8;
            let name = supply::read_adp_name(num);
            //let dev_type = supply::read_adp_devtype(num);
            let type_ = supply::read_adp_type(num);
            let online = if supply::read_adp_online(num) { "Yes" } else { "No" };

            power_supplies.push(PowerSupplyInfo {
                number: adp_number.to_string(),
                name,
                //dev_type,
                type_,
                online: online.to_string(),
            });
        }

        if power_supplies.is_empty() {
            println!("No power supply devices found.");
        } else {
            println!("{}", Table::new(power_supplies).to_string());
        }

        println!("\n{}","BAT Info".blue().bold());
        for bat_number in 1..bat_count+1 {
            let num = bat_number as u8;
            let name = battery::read_bat_name(num);
            //let dev_type = battery::read_bat_devtype(num);
            let type_ = battery::read_bat_type(num);
            let capacity = battery::read_bat_capacity(num);
            let capacity_lv = battery::read_bat_capacity_lv(num);
            let status = battery::read_bat_status(num);
            let model = battery::read_bat_model(num);
            let serialnum = battery::read_bat_serialnum(num);
            let manufacturer = battery::read_bat_manufacturer(num);
            let present_str = if battery::read_bat_present(num) { "Yes" } else { "No" };
            let technology = battery::read_bat_technology(num);
            
            battery_list.push(BatteryInfo{
                number: bat_number.to_string(),
                name,
                //dev_type,
                type_,
                capacity,
                capacity_lv,
                model ,
                serialnum,
                status,
                manufacturer,
                present: present_str.to_string(),
                technology,
            }); 
        }

        if battery_list.is_empty() {
            println!("No Battery devices found.");
        } else {
            println!("{}", Table::new(battery_list).to_string());
        }

        println!("\n{}","BAT Meta".blue().bold());
        for bat_number in 1..bat_count+1 {
            let num = bat_number as u8;
            let name = battery::read_bat_name(num);
            let charge_full_design = battery::read_bat_charge_full_design(num);
            let charge_now = battery::read_bat_charge_now(num);
            let current_now = battery::read_bat_current_now(num);
            let cyclecount = battery::read_bat_cyclecount(num);
            let volt_min = battery::read_bat_volt_min(num);
            let volt_now = battery::read_bat_volt_now(num);
            
            battery_meta.push(BatteryElectronMeta{
                number: bat_number.to_string(),
                name,
                charge_now,
                charge_full_design,
                cyclecount,
                volt_min,
                volt_now,
                current_now
            }); 
        }

        if battery_meta.is_empty() {
            println!("No Battery devices found.");
        } else {
            println!("{}", Table::new(battery_meta).to_string());
        }
    }

    println!("\n{}", "Disk Information".green().bold());
    println!("{}", "=================".green());
    println!("{}", "Sector Space:".blue().bold());
    let disk_info: Vec<DiskInfo> = sector_data.iter().map(|(name, filesystem, mount_point, total_space, used_space, used_point, free_space)| {
        DiskInfo {
            name: name.to_string(),
            filesystem: filesystem.as_deref().unwrap_or("Unknown").to_string(),
            mount_point: mount_point.as_deref().unwrap_or("Unknown").to_string(),
            total_space: format!("{:.2} GB", total_space),
            used_space: format!("{:.2} GB", used_space),
            used_percent: format!("{:.1}%", used_point),
            free_space: format!("{:.2} GB", free_space),
        }
    }).collect();
    println!("{}", Table::new(disk_info).to_string());

    println!("\n{}", "All Disks:".blue().bold());
    let mut disk_all_list = Vec::new();
    for (name, total_space) in disks_drive {
        disk_all_list.push(DiskAll{
            name: format!("{:<20}",name.to_string()),
            total_space: format!("{:.2} GB", total_space)
        })
    }
    println!("{}", Table::new(disk_all_list).to_string());


    println!("\n{}", "smartctl:".blue().bold());
    let mut disk_details = Vec::new();
    for name in disk_list {
        let path = format!("{}", name);
        disk_details.push(DiskDetails {
            path: path.clone(),
            status: disk::read_disk_smartstatus(&path),
            model: disk::read_disk_devicemodel(&path),
            firmware: disk::read_disk_firmware(&path),
            serial: disk::read_disk_serial(&path),
            factor: disk::read_disk_factor(&path),
            version: disk::read_disk_sataver(&path),
            rotation_rate: disk::read_disk_rotationrate(&path),
        });
    }
    println!("{}", Table::new(disk_details).to_string());
}