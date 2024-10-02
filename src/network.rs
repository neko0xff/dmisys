use crate::{
    cv::*,
    web::*
};
use std::{
    error::Error, 
    net::IpAddr
};
use get_if_addrs::get_if_addrs;
use sysinfo::Networks;

pub fn get_public_ipv4() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api.ipify.org";
    cn_server_get(url)
}

pub fn get_public_ipv6() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api6.ipify.org";
    cn_server_get(url)
}

pub fn get_public_ipv64() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api64.ipify.org";
    cn_server_get(url)
}

pub fn get_public_ipv4_address() -> String {
    format_msg(get_public_ipv4())
}

pub fn get_public_ipv6_address() -> String {
    format_msg(get_public_ipv6())
}

pub fn get_public_ipv64_address() -> String {
    format_msg(get_public_ipv64())
}

/*pub fn get_macaddress() -> Vec<(String, String)> {
    let mut mac_info = Vec::new();
    let mut mac_address = "None".to_string();
    let networks = Networks::new_with_refreshed_list();

    for (interface_name, network) in networks.iter() {
        let mac_address = match network.mac_address() {
            Some(mac) => {
                mac.to_string()
            },
            None => {
                "None".to_string()
            },
        };
        mac_info.push((interface_name.clone(), mac_address));
    }

    mac_info
}*/

pub fn get_local_ipv64() -> Vec<(String, String,String)> {
    let if_addrs = get_if_addrs().unwrap();
    let mut ip_info = Vec::new();
    let mut ipv4_addr = "None".to_string();
    let mut ipv6_addr = "None".to_string();

    for iface in if_addrs {
        let interface_name = iface.name.clone();
        if let IpAddr::V4(ipv4) = iface.addr.ip() {
            if !ipv4.is_loopback()  {
                ipv4_addr = ipv4.to_string();
            }
        }
        if let IpAddr::V6(ipv6) = iface.addr.ip() {
            if !ipv6.is_loopback() {
                ipv6_addr = ipv6.to_string();
            }
        }
        ip_info.push((interface_name, ipv4_addr.clone(),ipv6_addr.clone()));
    }

    if ip_info.is_empty() {
        ip_info.push(("".to_string(), "".to_string(),"".to_string()));
    }

    ip_info
}

pub fn get_speed() -> Vec<(String, f64, f64)> {
    let if_addrs = get_if_addrs().unwrap();
    let networks = Networks::new_with_refreshed_list();
    let mut speed_info = Vec::new();

    for iface in if_addrs {
        if let IpAddr::V4(ipv4) = iface.addr.ip() {
            if !ipv4.is_loopback() {
                if let Some((iface_name, data)) = networks.iter().find(|(name, _)| *name == &iface.name) {
                    let received_mb = bytes_to_mb(data.total_received()) as f64;
                    let transmitted_mb = bytes_to_mb(data.total_transmitted()) as f64;
                    speed_info.push((iface_name.to_string(), transmitted_mb, received_mb));
                }
            }
        }
    }

    if speed_info.is_empty() {
        speed_info.push(("No Found".to_string(), 0.0, 0.0));
    }

    speed_info
}
