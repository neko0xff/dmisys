use colored::*;
use dmisys::*;

pub fn output_msg() {
    println!("\n{}", "CPU Information".green().bold());
    println!("{}", "================".green());
    println!(
        "{:<20} {}",
        "CPU Model:".blue().bold(),
        cpu::read_cpu_model()
    );
    println!(
        "{:<20} {:.2} GHz",
        "CPU Frequency:".blue().bold(),
        cpu::get_cpu_frequency()
    );
    println!(
        "{:<20} {:?}",
        "CPU Core:".blue().bold(),
        cpu::read_cpu_cores()
    );
    println!(
        "{:<20} {:?}",
        "CPU Threads:".blue().bold(),
        cpu::read_cpu_threads()
    );
    println!(
        "{:<20} {}", 
        "CPU Arch:".blue().bold(), 
        cpu::read_cpu_arch()
    );
    println!(
        "{:<20} {:<.2}%",
        "CPU Load Avg:".blue().bold(),
        cpu::get_cpu_loading()
    );
}
