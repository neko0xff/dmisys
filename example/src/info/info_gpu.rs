use colored::*;
use dmisys::*;

pub fn output_msg() {
    let gpus = device::read_device_gpu();

    println!("\n{}", "GPU Information".green().bold());
    println!("{}", "================".green());
    println!("\n{}", "Installed GPU Device List".blue().bold());
    if gpus.is_empty() {
        println!("No GPUs found");
    } else {
        for (index, gpu) in gpus.iter().enumerate() {
            println!("{:<4} {}", format!("GPU {}:", index + 1).blue().bold(), gpu);
        }
    }
}
