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
    /// This function reads the total memory from `/proc/meminfo` and converts it to megabytes.
    /// It returns the total memory as a `f64` value.
    /// If the value is not found, it returns 0.0.
    pub fn total_memory(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("MemTotal").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Memory:  Available(MB)
    /// This function reads the available memory from `/proc/meminfo` and converts it to megabytes.
    /// It returns the available memory as a `f64` value.
    /// If the value is not found, it returns 0.0.
    pub fn available_memory(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("MemAvailable").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Memory: Used(MB)
    /// This function calculates the used memory by subtracting the available memory from the total memory.
    /// It returns the used memory as a `f64` value.
    /// If the total memory is not found, it returns 0.0.
    pub fn used_memory(&self) -> f64 {
        let total = self.total_memory();
        let available = self.available_memory();
        let used = total - available;

        used
    }

    /// Memory: Available(%)
    /// This function calculates the percentage of available memory by dividing the available memory by the total memory.
    /// It returns the percentage as a `f64` value.
    /// If the total memory is 0, it returns 0.0.
    pub fn available_memory_percent(&self) -> f64 {
        let available = self.available_memory();
        let total = self.total_memory();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(available, total)
        }
    }

    /// Memory: Used (%)
    /// This function calculates the percentage of used memory by dividing the used memory by the total memory.
    /// It returns the percentage as a `f64` value.
    /// If the total memory is 0, it returns 0.0.
    pub fn used_memory_percent(&self) -> f64 {
        let used = self.used_memory();
        let total = self.total_memory();
        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(used, total)
        }
    }

    ///  Swap: Total (MB)
    /// This function reads the total swap memory from `/proc/meminfo` and converts it to megabytes.
    /// It returns the total swap memory as a `f64` value.
    /// If the value is not found, it returns 0.0.
    pub fn total_swap(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("SwapTotal").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Swap: Available (MB)
    /// This function reads the available swap memory from `/proc/meminfo` and converts it to megabytes.
    /// It returns the available swap memory as a `f64` value.
    /// If the value is not found, it returns 0.0.
    pub fn available_swap(&self) -> f64 {
        Info::read_dir_meminfo()
            .and_then(|m| m.get("SwapFree").cloned())
            .map_or(0.0, |v| cv::bytes_to_mb(v * 1024))
    }

    /// Swap:  Used(MB)
    /// This function calculates the used swap memory by subtracting the available swap memory from the total swap memory.
    /// It returns the used swap memory as a `f64` value.
    /// If the total swap memory is not found, it returns 0.0.
    pub fn used_swap(&self) -> f64 {
        let total = self.total_swap();
        let available= self.available_swap();
        let used = total -  available;

        used
    }

    /// Swap: Used(%)
    /// This function calculates the percentage of used swap memory by dividing the used swap memory by the total swap memory.
    /// It returns the percentage as a `f64` value.
    /// If the total swap memory is 0, it returns 0.0.
    pub fn used_swap_percent(&self) -> f64 {
        let used = self.used_swap();
        let total = self.total_swap();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(used, total)
        }
    }

    /// Swap: Available(%)
    /// This function calculates the percentage of available swap memory by dividing the available swap memory by the total swap memory.
    /// It returns the percentage as a `f64` value.
    /// If the total swap memory is 0, it returns 0.0.
    pub fn available_swap_percent(&self) -> f64 {
        let free = self.available_swap();
        let total = self.total_swap();

        match total {
            0.0 => 0.0,
            _ => cv::percentage_cal(free, total)
        }
    }
}
