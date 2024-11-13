use chrono::prelude::*;
use colored::*;

pub fn time_cli() {
    let local_time = Local::now();
    let date_format = local_time.format("%Y-%m-%d");
    let time_format = local_time.format("%H:%M:%S");

    println!("\n{}", "Current  Times".green().bold());
    println!("{}", "===================".green());
    println!("Date: {}", date_format);
    println!("Time: {}", time_format);
}
