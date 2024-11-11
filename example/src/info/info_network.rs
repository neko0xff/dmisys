use crate::ds::ds_table::*;
use colored::*;
use dmisys::*;
use tabled::Table;

pub fn output_msg() {
    let local_ipv64 = network::get_local_ipv64();
    let network_speed = network::get_speed();
    let mac_address = network::get_macaddress();

    println!("\n{}", "Network".green().bold());
    println!("{}", "====================".green());
    println!("{}", "Public Address:".blue().bold());
    println!(
        "{:<15} {}",
        "IPv4:".yellow(),
        network::get_public_ipv4_address()
    );
    println!(
        "{:<15} {}",
        "IPv6:".yellow(),
        network::get_public_ipv6_address()
    );

    println!("\n{}", "Local Address:".blue().bold());
    if local_ipv64.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let network_info: Vec<NetworkInfo> = local_ipv64
            .iter()
            .map(|(interface, ipv4, ipv6)| NetworkInfo {
                interface: interface.to_string(),
                ipv4: ipv4.to_string(),
                ipv6: ipv6.to_string(),
            })
            .collect();
        println!("{}", Table::new(network_info));
    }

    println!("\n{}", "Network Speed:".blue().bold());
    if network_speed.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let speed_info: Vec<NetworkSpeed> = network_speed
            .iter()
            .map(|(interface, tx, rx)| NetworkSpeed {
                interface: interface.to_string(),
                tx_speed: format!("{} Mb", tx),
                rx_speed: format!("{} Mb", rx),
            })
            .collect();
        println!("{}", Table::new(speed_info));
    }

    println!("\n{}", "Mac Address:".blue().bold());
    if mac_address.is_empty() {
        println!("Not Running a Network Card!");
    } else {
        let mac_info: Vec<MacAddress> = mac_address
            .iter()
            .map(|(interface, mac)| MacAddress {
                interface: interface.to_string(),
                mac: mac.to_string(),
            })
            .collect();
        println!("{}", Table::new(mac_info));
    }
}
