use lib_dmisys::{bios, cpu, gpu, host, network, os, power,disk,memory,cv};
use sysinfo::{Networks, System};

fn main() {
    let host_vendor = host::read_host_vendor();
    let host_model = host::read_host_model();
    let host_boardname = host::read_host_boardname();
    let bios_date = bios::read_bios_date();
    let bios_vendor = bios::read_bios_vendor();
    let bios_version = bios::read_bios_version();
    let bios_release = bios::read_bios_release();
    let power_autosuspend_delay_ms = power::read_autosuspend_delay_ms();
    let power_control = power::read_control();
    let power_runtime_active_time = power::read_runtime_active_time();
    let power_runtime_status = power::read_runtime_status();
    let power_runtime_suspended_time = power::read_runtime_suspended_time();
    let sys = System::new_all();
    let load_avg = System::load_average();
    let gpu_line = gpu::read_gpu().trim().replace("VGA compatible controller:", "");
    
    let networks = Networks::new_with_refreshed_list();
    //std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    //sys.refresh_all();
    let (days,hours, minutes) = cv::read_systime_uptime();
    let (networkcard,ipv4_address) = network::get_local_ip();
    let (speed_networkcard,tx_speed,rx_speed) = network::get_speed();

    println!("\n System");
    println!("OS: {}", System::long_os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("Distro: {}",os::read_distro_name());
    println!("Host Name: {}", System::host_name().unwrap_or_else(|| "Unknown".to_string()));
    println!("Kernel: {}", System::kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("Vendor: {}", host_vendor);
    println!("Board: {}",host_boardname);
    println!("Model: {}", host_model);
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
    println!("CPU Model: {:?}",cpu::get_cpu_model().expect("REASON").to_string());
    println!("CPU Frequency(Ghz): {:.4} Ghz",cpu::get_cpu_frequency().expect("REASON").to_string());
    println!("CPU Core: {:?}",cpu::get_cpu_cores().expect("REASON"));
    println!("CPU threads: {:?}", sys.cpus().len());
    println!("CPU Arch: {}",System::cpu_arch().unwrap_or_else(|| "Unknown".to_string()));
    println!("CPU Load avg : {}%",load_avg.one);

    println!("\n GPU");
    println!("Device: {}", gpu_line);

    println!("\n BIOS");
    println!("Ventor: {}",bios_vendor);
    println!("Release: {}",bios_release);
    println!("Version: {}",bios_version);
    println!("Date: {}",bios_date);
    
    println!("\n Memory");
    println!("Use: {}MB", memory::used_memory());
    println!("Free: {}MB",memory::free_memory());
    println!("Total: {}MB", memory::total_memory());

    println!("\n Swap");
    println!("Use: {}MB", memory::used_swap());
    println!("Free: {}MB",memory::free_swap());
    println!("Total: {}MB", memory::total_swap());

    println!("\n Power");
    println!("Autosuspend Delay(ms): {} ms",power_autosuspend_delay_ms);
    println!("Control: {}",power_control);
    println!("Runtime Status: {}",power_runtime_status);
    println!("Runtime Active Time: {}",power_runtime_active_time);
    println!("Runtime Suspended Time: {}",power_runtime_suspended_time);

    println!("\n Disk");
    let (name, total_space) = disk::read_disk_totalspace();
    println!("{}:{}",name,total_space);
    match disk::read_disk_smartinfo("/dev/sda") {
        Ok(info) => println!("SMART Info:\n{}", info),
        Err(e) => println!("Failed to get SMART info: {}", e),
    }

    
}
