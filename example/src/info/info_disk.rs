use crate::ds::ds_table::*;
use dmisys::*;
use colored::*;
use tabled::Table;

pub fn output_msg() {
    let disks_drive = disk::read_disk_all_vec();
    let sector_data = disk::read_disk_sectorspace_vec();
    let disk_list = disk::read_disks_physicaldrive_list();

    println!("\n{}", "Disk Information".green().bold());
    println!("{}", "=================".green());
    println!("{}", "Sector Space:".blue().bold());
    let disk_info: Vec<DiskInfo> = sector_data.iter().map(|(name, filesystem, mount_point, total_space, used_space, used_point, free_space)| {
        DiskInfo {
            name: name.to_string(),
            filesystem: filesystem.as_deref().unwrap_or("Unknown").to_string(),
            mount_point: mount_point.as_deref().unwrap_or("Unknown").to_string(),
            total_space: format!("{:.2} GB", total_space),
            used_space: format!("{:.2} GB", used_space),
            used_percent: format!("{:.1}%", used_point),
            free_space: format!("{:.2} GB", free_space),
        }
    }).collect();
    println!("{}", Table::new(disk_info).to_string());

    println!("\n{}", "All Disks:".blue().bold());
    let mut disk_all_list = Vec::new();
    for (name, total_space) in disks_drive {
        disk_all_list.push(DiskAll{
            name: format!("{:<20}",name.to_string()),
            total_space: format!("{:.2} GB", total_space)
        })
    }
    println!("{}", Table::new(disk_all_list).to_string());


    println!("\n{}", "smartctl:".blue().bold());
    let mut disk_details = Vec::new();
    for name in disk_list {
        let path = format!("{}", name);
        disk_details.push(DiskDetails {
            path: path.clone(),
            status: disk::read_disk_smartstatus(&path),
            model: disk::read_disk_devicemodel(&path),
            firmware: disk::read_disk_firmware(&path),
            serial: disk::read_disk_serial(&path),
            factor: disk::read_disk_factor(&path),
            version: disk::read_disk_sataver(&path),
            rotation_rate: disk::read_disk_rotationrate(&path),
        });
    }
    println!("{}", Table::new(disk_details).to_string());
}