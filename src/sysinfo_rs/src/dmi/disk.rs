use std::process::Command;
use sysinfo::Disks;

pub fn bytes_to_gb(bytes: u64) -> f64 {
    let cv_gb = f64::powf(2 as f64,30 as f64);
    (bytes as f64) / cv_gb
}

pub fn read_disk_smartinfo(device: &str) -> Result<String, std::io::Error> {
    let output = Command::new("sudo")
        .arg("smartctl")
        .arg("-a") 
        .arg(device) 
        .output()?;
    let smart_info = String::from_utf8_lossy(&output.stdout).to_string();
    
    Ok(smart_info)
}

pub fn read_disk_totalspace() -> (String, f64) {
    let disks = Disks::new_with_refreshed_list();
    let mut name = String::new(); 
    let mut total_space = 0.0; 

    disks.list().into_iter().for_each(|disk| {
        name = disk.name().to_string_lossy().to_string();
        total_space = bytes_to_gb(disk.total_space()) ; 
    });

    if name.is_empty() {
        name = "Not Found".to_string();
        total_space = 0.0;
    }

    (name, total_space)
}