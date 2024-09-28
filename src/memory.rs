use sysinfo::System;
use crate::cv::*;

// 回傳單位: Mbyte

pub struct Info {
    pub sys: System,
}

impl Info {
    pub fn new() -> Self {
        let sys = System::new_all();
        Info { sys }
    }

    // 己分配的RAM空間
    pub fn total_memory(&self) -> u64 {
        bytes_to_mb(self.sys.total_memory())
    }
  
    // 己使用的RAM空間
    pub fn used_memory(&self) -> u64 {
        bytes_to_mb(self.sys.used_memory())
    }
    pub fn used_memory_percent(&self) -> f64 {
        let used_memory = self.used_memory() as f64;
        let total_memory = self.total_memory() as f64;
    
        if total_memory == 0.0 {
            0.0 
        } else {
            (used_memory / total_memory) * 100.0
        }
    }
    
    // 未使用的RAM空間
    pub fn free_memory(&self) -> u64 {
        bytes_to_mb(self.sys.free_memory())
    }
    pub fn free_memory_percent(&self) -> f64 {
        let free_memory = self.free_memory() as f64;
        let total_memory = self.total_memory() as f64;
    
        if total_memory == 0.0 {
            0.0 // 防止除以 0 的情況
        } else {
            (free_memory / total_memory) * 100.0
        }
    }
    
    // 可供於程式使用的RAM空間
    pub fn available_memory(&self) -> u64 {
        bytes_to_mb(self.sys.available_memory())
    }
    pub fn available_memory_percent(&self) -> f64 {
        let available_memory = self.available_memory() as f64;
        let total_memory = self.total_memory() as f64;
    
        if total_memory == 0.0 {
            0.0 // 防止除以 0 的情況
        } else {
            (available_memory / total_memory) * 100.0
        }
    }
    
    // 己分配的暫存交換區空間
    pub fn total_swap(&self) -> u64 {
        bytes_to_mb(self.sys.total_swap())
    }

     // 未使用的暫存交換區空間
    pub fn free_swap(&self) -> u64 {
        bytes_to_mb(self.sys.free_swap())
    }
    pub fn free_swap_percent(&self) -> f64 {
        let free_swap = self.free_swap() as f64;
        let total_swap = self.total_swap() as f64;
    
        if total_swap == 0.0 {
            0.0
        } else {
            (free_swap / total_swap) * 100.0
        }
    }

    // 己使用的暫存交換區空間
    pub fn used_swap(&self) -> u64 {
        bytes_to_mb(self.sys.used_swap())
    }
    pub fn used_swap_percent(&self) -> f64 {
        let used_swap = self.used_swap() as f64;
        let total_swap = self.total_swap() as f64;
    
        if total_swap == 0.0 {
            0.0
        } else {
            (used_swap / total_swap) * 100.0
        }
    }
}
