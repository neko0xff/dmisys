use crate::cv;
use sysinfo::System;

pub struct Info {
    pub sys: System,
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    pub fn new() -> Self {
        let sys = System::new_all();
        Info { sys }
    }

    /// total installed memory space(MB)
    pub fn total_memory(&self) -> u64 {
        cv::bytes_to_mb(self.sys.total_memory())
    }

    /// used installed memory space (MB)
    pub fn used_memory(&self) -> u64 {
        cv::bytes_to_mb(self.sys.used_memory())
    }

    /// used installed memory space (percentage)
    pub fn used_memory_percent(&self) -> f64 {
        let used = self.used_memory() as f64;
        let total = self.total_memory() as f64;

        if total == 0.0 {
            0.0
        } else {
            cv::percentage_cal(used, total)
        }
    }

    /// free installed memory space (MB)
    pub fn free_memory(&self) -> u64 {
        cv::bytes_to_mb(self.sys.free_memory())
    }

    /// free installed memory space (percentage)
    pub fn free_memory_percent(&self) -> f64 {
        let free_memory = self.free_memory() as f64;
        let total_memory = self.total_memory() as f64;

        if total_memory == 0.0 {
            0.0
        } else {
            cv::percentage_cal(free_memory, total_memory)
        }
    }

    /// available installed memory space (MB)
    pub fn available_memory(&self) -> u64 {
        cv::bytes_to_mb(self.sys.available_memory())
    }

    /// available installed memory space (percentage)
    pub fn available_memory_percent(&self) -> f64 {
        let available_memory = self.available_memory() as f64;
        let total_memory = self.total_memory() as f64;

        if total_memory == 0.0 {
            0.0
        } else {
            cv::percentage_cal(available_memory, total_memory)
        }
    }

    /// total swap space (MB)
    pub fn total_swap(&self) -> u64 {
        cv::bytes_to_mb(self.sys.total_swap())
    }

    /// Free swap space
    pub fn free_swap(&self) -> u64 {
        cv::bytes_to_mb(self.sys.free_swap())
    }

    /// Free swap space (percentage)
    pub fn free_swap_percent(&self) -> f64 {
        let free_swap = self.free_swap() as f64;
        let total_swap = self.total_swap() as f64;

        if total_swap == 0.0 {
            0.0
        } else {
            cv::percentage_cal(free_swap, total_swap)
        }
    }

    /// Used a swap space
    pub fn used_swap(&self) -> u64 {
        cv::bytes_to_mb(self.sys.used_swap())
    }

    /// Used a swap space (percentage)
    pub fn used_swap_percent(&self) -> f64 {
        let used = self.used_swap() as f64;
        let total = self.total_swap() as f64;

        if total == 0.0 {
            0.0
        } else {
            cv::percentage_cal(used, total)
        }
    }
}
