use colored::*;
use dmisys::*;

pub fn output_msg() {
    let gpus = device::read_device_gpu();
    let opengl_version = opengl::read_opengl_version();
    let opengl_vendor = opengl::read_opengl_vendor();
    let opengl_renderer = opengl::read_opengl_rendererdevice();

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

    println!("\n{}", "OpenGL".green().bold());
    println!("{}", "================".green());
    println!(
        "{:<15} {}",
        "Version:".blue().bold(),
        opengl_version
    );
    println!(
        "{:<15} {}",
        "Vendor:".blue().bold(),
        opengl_vendor
    );
    println!("\n{}", "Renderer ".green().bold());
    println!("{}", "================".green());
    println!(
        "{:<15} {}",
        "Device:".blue().bold(),
        opengl_renderer
    );
    println!(
        "{:<15} {} MB",
        "Video RAM:".blue().bold(),
        opengl::read_renderer_videoram()
    );
    println!(
        "{:<15} {}",
        "Direct:".blue().bold(),
        opengl::read_renderer_direct()
    );
    println!(
        "{:<15} {}",
        "Accelerated:".blue().bold(),
        opengl::read_renderer_accelerated()
    );
    println!(
        "{:<15} {}",
        "Share Memory:".blue().bold(),
        opengl::read_renderer_sharemenory()
    );

    println!("\n{}", "Mesa".green().bold());
    println!("{}", "================".green());
    println!(
        "{:<15} {}",
        "Version:".blue().bold(),
        opengl::read_mesa_ver()
    );
   
}
