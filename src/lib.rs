/*!

This is a specialized library designed to output the hardware configuration required by the system,
as well as various status information of the current device.

Specifically, it can provide crucial metrics such as network upload and download speeds, local IP address, and public IP address. This information is essential for monitoring system performance,
troubleshooting issues,
and optimizing the network environment.

# Example:  CPU Information

This example shows how to output CPU hardware space.

```
use dmisys::*;

fn main() {
    println!("CPU Model: {:?}", cpu::read_cpu_model());
    println!("CPU Frequency: {:.2} GHz", cpu::get_cpu_frequency());
    println!("CPU Core: {:?}", cpu::read_cpu_cores());
    println!("CPU Threads: {:?}", cpu::read_cpu_threads());
    println!("CPU Arch: {}", cpu::read_cpu_arch());
    println!("CPU Load Avg: {}%",  cpu::get_cpu_loading().to_string());
}
```

# Example:  GPU Device List

This example shows how to output Installed GPU Devices List

```
use dmisys::*;

pub fn output_msg() {
    let gpus = device::read_device_gpu();

    if gpus.is_empty() {
        println!("No GPUs found");
    } else {
        for (index, gpu) in gpus.iter().enumerate() {
            let count = index+1;
            println!("GPU {}: {}",  count, gpu);
        }
    }
}
```

 */

pub mod battery;
pub mod bios;
pub mod cpu;
pub mod cv;
pub mod device;
pub mod devtools;
pub mod disk;
pub mod display;
pub mod env;
pub mod file;
pub mod host;
pub mod memory;
pub mod network;
pub mod os;
pub mod power;
pub mod supply;
pub mod systime;
pub mod web;
pub mod opengl;
pub mod audio;