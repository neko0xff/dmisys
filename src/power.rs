use std::fs;

pub fn read_autosuspend_delay_ms() -> String {
    let autosuspend_delay_ms = fs::read_to_string("/sys/class/dmi/id/power/autosuspend_delay_ms")
        .unwrap_or_else(|_| "Unknown".to_string());

    format!("{}", autosuspend_delay_ms.trim())
}

pub fn read_control() -> String {
    let control = fs::read_to_string("/sys/class/dmi/id/power/control")
        .unwrap_or_else(|_| "Unknown".to_string());

    format!("{}", control.trim())
}

pub fn read_runtime_active_time() -> String {
    let runtime_active_time = fs::read_to_string("/sys/class/dmi/id/power/runtime_active_time")
        .unwrap_or_else(|_| "Unknown".to_string());

    format!("{}", runtime_active_time.trim())
}

pub fn read_runtime_status() -> String {
    let runtime_status = fs::read_to_string("/sys/class/dmi/id/power/runtime_status")
        .unwrap_or_else(|_| "Unknown".to_string());

    format!("{}", runtime_status.trim())
}

pub fn read_runtime_suspended_time() -> String {
    let runtime_suspended_time = fs::read_to_string("/sys/class/dmi/id/power/runtime_suspended_time")
        .unwrap_or_else(|_| "Unknown".to_string());
    
    format!("{}", runtime_suspended_time.trim())
}