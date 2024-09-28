use lib_dmisys::{
    bios,
    cpu, 
    gpu, 
    host, 
    network, 
    os, 
    power,
    disk,
    memory,
    cv,
    systime
};
use sysinfo::{
    Networks, 
    System
};

fn main() {
    let memory_read = memory::Info::new();
    let networks = Networks::new_with_refreshed_list();
    let (days,hours, minutes) = systime::read_systime_uptime();
    let (networkcard,ipv4_address) = network::get_local_ip();
    let (speed_networkcard,tx_speed,rx_speed) = network::get_speed();

    println!("\n System");
    println!("OS: {}",os::read_osname());
    println!("Distro: {}",os::read_distro_name());
    println!("Host Name: {}", os::read_hostname());
    println!("Kernel: {}", os::read_kernel());
    println!("Vendor: {}", host::read_host_vendor());
    println!("Board: {}",host::read_host_boardname());
    println!("Model: {}", host::read_host_model());
    println!("Uptime: {} days, {} hours, {} minutes", days, hours, minutes);

    println!("\n Network");
    println!("Public IP Address: {}", network::get_public_ip().expect("REASON").to_string());
    println!("Local IP Address:");
    println!("{} : {}",networkcard,ipv4_address);
    println!("\n Network Speed");
    networks.into_iter().for_each(|(interface_name, data)| {
        let received_mb = cv::bytes_to_mb(data.total_received()) as f64;
        let transmitted_mb = cv::bytes_to_mb(data.total_transmitted()) as f64;
        println!("{}: Rx {}MB / Tx {}MB", interface_name, received_mb, transmitted_mb);
    });
    println!("\n Speed");
    println!("{}: Tx = {} Mb / Rx = {} Mb",speed_networkcard,tx_speed,rx_speed);
    println!("{}: Tx = {} Mb / Rx = {} Mb",speed_networkcard,tx_speed,rx_speed);

    println!("\n CPU");
    println!("CPU Model: {:?}",cpu::get_cpu_model());
    println!("CPU Frequency(Ghz): {:.4} Ghz",cpu::get_cpu_frequency());
    println!("CPU Core: {:?}",cpu::get_cpu_cores());
    println!("CPU threads: {:?}", cpu::read_cpu_threads());
    println!("CPU Arch: {}",cpu::read_cpu_arch());
    println!("CPU Load avg : {}%",cpu::get_cpu_loading().to_string());

    println!("\n GPU");
    println!("Device: {}",  gpu::read_gpu_device());

    println!("\n BIOS");
    println!("Ventor: {}",bios::read_bios_vendor());
    println!("Release: {}",bios::read_bios_release());
    println!("Version: {}",bios::read_bios_version());
    println!("Date: {}",bios::read_bios_date());
    
    println!("\n Memory");
    println!("Use: {}MB ({:.2}%)", memory_read.used_memory(),memory_read.used_memory_percent());
    println!("Free: {}MB ({:.2}%)",memory_read.free_memory(),memory_read.free_memory_percent());
    println!("Available: {}MB ({:.2}%)", memory_read.available_memory(),memory_read.available_memory_percent());
    println!("Total: {}MB", memory_read.total_memory());

    println!("\n Swap");
    println!("Use: {}MB ({:.2}%)", memory_read.used_swap(),memory_read.used_swap_percent());
    println!("Free: {}MB ({:.2}%)",memory_read.free_swap(),memory_read.free_swap_percent());
    println!("Total: {}MB", memory_read.total_swap());

    println!("\n Power");
    println!("Autosuspend Delay(ms): {} ms",power::read_autosuspend_delay_ms());
    println!("Control: {}",power::read_control());
    println!("Runtime Status: {}",power::read_runtime_status());
    println!("Runtime Active Time: {}",power::read_runtime_active_time());
    println!("Runtime Suspended Time: {}",power::read_runtime_suspended_time());

    println!("\n Disk");
    let (name, total_space) = disk::read_disk_totalspace();
    println!("{}:{}",name,total_space);
    match disk::read_disk_smartinfo("/dev/sda") {
        Ok(info) => println!("SMART Info:\n{}", info),
        Err(e) => println!("Failed to get SMART info: {}", e),
    }

}
