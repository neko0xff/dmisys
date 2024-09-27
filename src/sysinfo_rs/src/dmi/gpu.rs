use std::process::Command;

pub fn read_gpu() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d\\|2d'")
        .output()
        .expect("Failed to execute command");
    let gpu_info = String::from_utf8_lossy(&output.stdout);

    gpu_info.to_string()
}