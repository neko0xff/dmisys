use colored::*;
use dmisys::*;

pub fn output_msg() {
    let (days_up, hours_up, minutes_up) = systime::read_systime_up();
    //let (days_unix, hours_unix, minutes_unix) = systime::read_systime_boot();
    let (io_write, io_read) = os::read_io_speed();

    println!("\n{}", "System Information".green().bold());
    println!("{}", "=================".green());
    println!("{:<15} {}", "OS:".blue().bold(), os::read_osname());
    println!("{:<15} {}", "Distro:".blue().bold(), os::read_distro_name());
    println!("{:<15} {}", "Host Name:".blue().bold(), os::read_hostname());
    println!("{:<15} {}", "Kernel:".blue().bold(), os::read_kernel());
    println!(
        "{:<15} {}",
        "Vendor:".blue().bold(),
        host::read_host_vendor()
    );
    println!(
        "{:<15} {}",
        "Board:".blue().bold(),
        host::read_host_boardname()
    );
    println!("{:<15} {}", "Model:".blue().bold(), host::read_host_model());
    println!(
        "{:<15} {} days, {} hours, {} minutes",
        "Uptime:".blue().bold(),
        days_up,
        hours_up,
        minutes_up
    );
    //println!("{:<15} {} days, {} hours, {} minutes", "Unix time:".blue().bold(), days_unix, hours_unix, minutes_unix);
    println!(
        "{:<15} Write = {} MB / Read = {} MB",
        "IO:".blue().bold(),
        io_write,
        io_read
    );

    println!("\n{}", "BIOS Information".green().bold());
    println!("{}", "=================".green());
    println!(
        "{:<15} {}",
        "Vendor:".blue().bold(),
        bios::read_bios_vendor()
    );
    println!(
        "{:<15} {}",
        "Release:".blue().bold(),
        bios::read_bios_release()
    );
    println!(
        "{:<15} {}",
        "Version:".blue().bold(),
        bios::read_bios_version()
    );
    println!("{:<15} {}", "Date:".blue().bold(), bios::read_bios_date());
}
