use crate::cv;
use std::{
    collections::HashMap,
    fs
};

pub struct Info;

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    pub fn new() -> Self {
        Info
    }

    /// read /proc/meminfo
    fn read_dir_meminfo() -> Option<HashMap<String, u64>> {
        let content = fs::read_to_string("/proc/meminfo").ok()?;
        let mut meminfo = HashMap::new();
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let value = value.trim().split_whitespace().next()?.parse::<u64>().ok()?;
                meminfo.insert(key.to_string(), value);
            }
        }

        match  meminfo.is_empty() {
            true => None,
            false => Some(meminfo)
        }
    }

    /// Memory: Total(MB)
    pub fn total_memory(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("MemTotal").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Memory:  Available(MB)
    pub fn available_memory(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("MemAvailable").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Memory: Used(MB)
    pub fn used_memory(&self) -> f64 {
        let total = self.total_memory();
        let available = self.available_memory();
        let used = total - available;

        used
    }

    /// Memory: Available(%)
    pub fn available_memory_percent(&self) -> f64 {
        let available = self.available_memory();
        let total = self.total_memory();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(available, total)
        }
    }

    /// Memory: Used (%)
    pub fn used_memory_percent(&self) -> f64 {
        let used = self.used_memory();
        let total = self.total_memory();
        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(used, total)
        }
    }

    ///  Swap: Total (MB)
    pub fn total_swap(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("SwapTotal").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Swap: Available (MB)
    pub fn available_swap(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("SwapFree").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Swap:  Used(MB)
    pub fn used_swap(&self) -> f64 {
        let total = self.total_swap();
        let available= self.available_swap();
        let used = total -  available;

        used
    }

    /// Swap: Used(%)
    pub fn used_swap_percent(&self) -> f64 {
        let used = self.used_swap();
        let total = self.total_swap();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(used, total)
        }
    }

    /// Swap: Available(%)
    pub fn available_swap_percent(&self) -> f64 {
        let free = self.available_swap();
        let total = self.total_swap();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(free, total)
        }
    }
}
