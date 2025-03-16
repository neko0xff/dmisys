use colored::*;
use dmisys::*;

pub fn output_msg() {
    let memory_read = memory::Info::new();

    println!("\n{}", "Memory Information".green().bold());
    println!("{}", "===================".green());
    println!(
        "{:<15} {:.2}MB ({:.2}%)",
        "Used:".blue().bold(),
        memory_read.used_memory(),
        memory_read.used_memory_percent()
    );
    println!(
        "{:<15} {:.2}MB ({:.2}%)",
        "Free:".blue().bold(),
        memory_read.free_memory(),
        memory_read.free_memory_percent()
    );
    println!(
        "{:<15} {:.2}MB ({:.2}%)",
        "Available:".blue().bold(),
        memory_read.available_memory(),
        memory_read.available_memory_percent()
    );
    println!(
        "{:<15} {:.2}MB",
        "Total:".blue().bold(),
        memory_read.total_memory()
    );

    println!("\n{}", "Swap Information".green().bold());
    println!("{}", "=================".green());
    println!(
        "{:<15} {:.2}MB ({:.2}%)",
        "Used:".blue().bold(),
        memory_read.used_swap(),
        memory_read.used_swap_percent()
    );
    println!(
        "{:<15} {:.2}MB ({:.2}%)",
        "Free:".blue().bold(),
        memory_read.free_swap(),
        memory_read.free_swap_percent()
    );
    println!(
        "{:<15} {:.2}MB",
        "Total:".blue().bold(),
        memory_read.total_swap()
    );
}
