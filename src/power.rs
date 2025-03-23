use crate::file;

/// Read the maximum sleep time (in milliseconds) for the system to suspend
pub fn read_autosuspend_delay_ms() -> String {
    file::read_dmi_path("power/autosuspend_delay_ms")
}

/// Read the system's power management mode
pub fn read_control() -> String {
    file::read_dmi_path("power/control")
}

/// Read runtime_active_time in seconds
pub fn read_runtime_active_time() -> String {
    file::read_dmi_path("power/runtime_active_time")
}

/// Read runtime_status in milliseconds
pub fn read_runtime_status() -> String {
    file::read_dmi_path("power/runtime_status")
}

/// Read runtime_suspended_time in seconds
pub fn read_runtime_suspended_time() -> String {
    file::read_dmi_path("power/runtime_suspended_time")
}
