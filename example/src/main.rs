use dmisys::{
    bios,
    cpu, 
    device, 
    host, 
    network, 
    os, 
    power,
    disk,
    memory,
    systime,
    supply
};

fn main() {
    let memory_read = memory::Info::new();
    let gpus = device::read_device_gpu();
    let disks_drive = disk::read_disk_all_vec();
    let sector_data = disk::read_disk_sectorspace_vec();
    let disk_list = disk::read_disks_physicaldrive_list();
    let (days_up,hours_up, minutes_up) = systime::read_systime_up();
    let (days_unix,hours_unix, minutes_unix) = systime::read_systime_boot();
    let (io_write,io_read) = os::read_io_speed();
    let local_ipv64 = network::get_local_ipv64();
    let network_speed = network::get_speed();
    let mac_address = network::get_macaddress();

    println!("\n System");
    println!("OS: {}",os::read_osname());
    println!("Distro: {}",os::read_distro_name());
    println!("Host Name: {}", os::read_hostname());
    println!("Kernel: {}", os::read_kernel());
    println!("Vendor: {}", host::read_host_vendor());
    println!("Board: {}",host::read_host_boardname());
    println!("Model: {}", host::read_host_model());
    println!("Uptime: {} days, {} hours, {} minutes", days_up, hours_up, minutes_up);
    println!("Unix time(In 1970/01/01): {} days, {} hours, {} minutes", days_unix, hours_unix, minutes_unix);
    println!("IO: Write= {} MB/Read= {} MB",io_write,io_read);

    println!("\n Network");
    println!("Public Address");
    println!(" IPv4 Address: {}", network::get_public_ipv4_address());
    println!(" IPv6 Address: {}", network:: get_public_ipv6_address());
    println!("Local Address");
    if local_ipv64.is_empty(){
        println!("Not Running a Network Card!");
    }else{
        for(_index,(interface_name,local_ipv4,local_ipv6)) in local_ipv64.iter().enumerate(){
            println!("{}: IPV4  = {} / IPv6 = {}",interface_name,local_ipv4,local_ipv6);
        }
    }

    println!("\n Network Speed");
    if network_speed.is_empty(){
        println!("Not Running a Network Card!");
    }else{
        for(_index,(interface_name,tx_speed,rx_speed)) in network_speed.iter().enumerate(){
            println!("{}: Tx = {} Mb / Rx = {} Mb",interface_name,tx_speed,rx_speed);
        }
    }

    println!("\n Mac Address");
    if mac_address.is_empty(){
        println!("Not Running a Network Card!");
    }else{
        for(_index,(interface_name,mac)) in mac_address.iter().enumerate(){
            println!("{}: {}",interface_name,mac);
        }
    }
    
    println!("\n CPU");
    println!("CPU Model: {:?}",cpu::read_cpu_model());
    println!("CPU Frequency(Ghz): {:.4} Ghz",cpu::get_cpu_frequency());
    println!("CPU Core: {:?}",cpu::read_cpu_cores());
    println!("CPU threads: {:?}", cpu::read_cpu_threads());
    println!("CPU Arch: {}",cpu::read_cpu_arch());
    println!("CPU Load avg : {}%",cpu::get_cpu_loading().to_string());

    println!("\n GPU");
    if gpus.is_empty() {
        println!("No GPUs found");
    } else {
        for (index, gpu) in gpus.iter().enumerate() {
            println!("GPU {}: {}", index + 1, gpu);
        }
    }

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

    println!("\n Power Supply");
    println!("Main: {}",supply::read_main_name());
    println!("\tDev Type: {}",supply::read_main_devtype());
    println!("\tStatus: {}",supply::read_main_online());

    println!("\n Disk");
    println!("Sector Space");
    for (index, (name, filesystem, mount_point, total_space, used_space,used_point, free_space)) in sector_data.iter().enumerate() {
        println!(
            "Sector {}: {} (Filesystem: {}, Mount: {})",
            index,
            name,
            filesystem.as_deref().unwrap_or("Unknown"),
            mount_point.as_deref().unwrap_or("Unknown")
        );
        println!(
            "    Total: {:.2} GB, Used: {:.2} GB ({:.1}%), Free: {:.2} GB",
            total_space,
            used_space,
            used_point,
            free_space
        );
    }
    println!("All Disk");
    for (name, total_space) in disks_drive {
        println!(" {}: {:.2} GB", name, total_space);
    }

    println!("Disk Info");
    for name in disk_list {
        let path = format!("{}",name);
        println!(" {}",path);
        println!("\tStatus: {}",disk::read_disk_smartstatus(&path));
        println!("\tModel: {}",disk::read_disk_devicemodel(&path));
        println!("\tFirmware: {}",disk::read_disk_firmware(&path));
        println!("\tVersion: {}",disk::read_disk_sataver(&path));
        println!("\tRotation Rate: {}",disk::read_disk_rotationrate(&path));
    }
   
}
