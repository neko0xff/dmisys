use crate::{
    cv, 
    web
};
use std::{
    error::Error,
    net::{
        IpAddr, 
        Ipv4Addr, 
        Ipv6Addr
    },
    fs::File,
    fs,
    path::Path,
    io::{
        BufRead, 
        BufReader
    },
    ffi::CStr,
    ptr
};
use libc::{
    getifaddrs,
    freeifaddrs,
    ifaddrs, 
    sockaddr, 
    AF_INET, 
    AF_INET6
};
use get_if_addrs::get_if_addrs;


fn get_public_ipv4() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api.ipify.org";
    web::cn_server_get(url)
}

fn get_public_ipv6() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api6.ipify.org";
    web::cn_server_get(url)
}

fn get_public_ipv64() -> Result<Option<String>, Box<dyn Error>> {
    let url = "https://api64.ipify.org";
    web::cn_server_get(url)
}

/// Get Public IPv4 address
pub fn get_public_ipv4_address() -> String {
    cv::format_msg(get_public_ipv4())
}

/// Get Public IPv6 address
pub fn get_public_ipv6_address() -> String {
    cv::format_msg(get_public_ipv6())
}

/// Get Public IPv4 or IPv4 address
pub fn get_public_ipv64_address() -> String {
    cv::format_msg(get_public_ipv64())
}

/// Get Local IPv4 & IPv6 address
pub fn get_local_ipv64() -> Vec<(String, String, String)> {
    let if_addrs = get_if_addrs().unwrap();
    let mut ip_info = Vec::new();
    let mut ipv4_addr = "None".to_string();
    let mut ipv6_addr = "None".to_string();

    for iface in if_addrs {
        let interface_name = iface.name.clone();
        if let IpAddr::V4(ipv4) = iface.addr.ip() {
            if !ipv4.is_loopback() {
                ipv4_addr = ipv4.to_string();
            }
        }
        if let IpAddr::V6(ipv6) = iface.addr.ip() {
            if !ipv6.is_loopback() {
                ipv6_addr = ipv6.to_string();
            }
        }
        ip_info.push((interface_name, ipv4_addr.clone(), ipv6_addr.clone()));
    }

    if ip_info.is_empty() {
        ip_info.push(("".to_string(), "".to_string(), "".to_string()));
    }

    ip_info
}

/// Get Network interfaces & Address
pub fn get_network_interfaces() -> Vec<(String, IpAddr)> {
    let mut interfaces = Vec::new();

    unsafe {
        let mut ifap: *mut ifaddrs = ptr::null_mut();

        if getifaddrs(&mut ifap) == 0 {
            let mut ptr = ifap;

            while !ptr.is_null() {
                let iface = &*ptr;

                if let Some(name) = iface.ifa_name.as_ref() {
                    let iface_name = CStr::from_ptr(name).to_string_lossy().into_owned();

                    // Read IPv4 & IPv6 Address
                    if !iface.ifa_addr.is_null() {
                        let sockaddr = &*(iface.ifa_addr as *const sockaddr);
                        let ip_addr = match sockaddr.sa_family as i32 {
                            AF_INET => {
                                let sin = *(iface.ifa_addr as *const libc::sockaddr_in);
                                IpAddr::V4(Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr)))
                            }
                            AF_INET6 => {
                                let sin6 = *(iface.ifa_addr as *const libc::sockaddr_in6);
                                IpAddr::V6(Ipv6Addr::from(sin6.sin6_addr.s6_addr))
                            }
                            _ => continue,
                        };

                        interfaces.push((iface_name, ip_addr));
                    }
                }

                ptr = iface.ifa_next;
            }

            freeifaddrs(ifap);
        }
    }

    interfaces
}


/// Get now network interfaces a Upload & Download speed
fn read_net_stat(interface: &str, stat: &str) -> Option<f64> {
    let path = format!("/sys/class/net/{}/statistics/{}", interface, stat);

    if Path::new(&path).exists() {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok())
    } else {
        None
    }
}

/// Get Network received & transmitted Speed
pub fn get_speed() -> Vec<(String, f64, f64)> {
    let mut speed_info = Vec::new();
    let dir_path = "/sys/class/net";
    let interfaces = fs::read_dir(dir_path) 
        .ok()
        .into_iter()
        .flatten(); 

    for iface in interfaces {
        if let Ok(entry) = iface {
            let iface_name = entry.file_name().into_string().unwrap();
            let received_bytes = read_net_stat(&iface_name, "rx_bytes").unwrap_or(0.0);
            let transmitted_bytes = read_net_stat(&iface_name, "tx_bytes").unwrap_or(0.0);

            let received_mb = cv::bytes_to_mb(received_bytes as u64);
            let transmitted_mb = cv::bytes_to_mb(transmitted_bytes as u64);

            speed_info.push((iface_name, transmitted_mb, received_mb));
        }
    }

    if speed_info.is_empty() {
        speed_info.push(("Not Found".to_string(), 0.0, 0.0));
    }

    speed_info
}


/// Get MAC address of network interfaces
fn get_mac_address(interface: &str) -> Option<String> {
    let path = format!("/sys/class/net/{}/address", interface);

    if Path::new(&path).exists() {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

/// Get Network interface a Mac Address
pub fn get_macaddress() -> Vec<(String, String)> {
    let mut mac_info = Vec::new();
    let dir_path = "/sys/class/net";
    let interfaces = fs::read_dir(dir_path) 
        .ok()
        .into_iter()
        .flatten(); 

    for iface in interfaces {
        if let Ok(entry) = iface {
            let iface_name = entry.file_name().into_string().unwrap();
            if let Some(mac) = get_mac_address(&iface_name) {
                mac_info.push((iface_name, mac));
            }
        }
    }

    if mac_info.is_empty() {
       mac_info.push(("Not Found".to_string(), "".to_string()));
    }

    mac_info
}


/// Get DNS nameservers
pub fn get_nameservers() -> Vec<String> {
    let nameserver =  "/etc/resolv.conf";

    match File::open(nameserver) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .filter_map(Result::ok)
                .filter(|line| line.starts_with("nameserver"))
                .filter_map(|line| {
                    line.split_whitespace()
                        .nth(1)
                        .map(String::from)
                })
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
