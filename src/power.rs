use crate::file;

/// Read the maximum sleep time (in milliseconds) for the system to suspend
pub fn read_autosuspend_delay_ms() -> String {
    let dmi = "/sys/class/dmi/id/power/autosuspend_delay_ms";

    file::return_pathdata(dmi)
}

/// Read the system's power management mode
pub fn read_control() -> String {
    let dmi = "/sys/class/dmi/id/power/control";

    file::return_pathdata(dmi)
}

/// Read runtime_active_time in seconds
pub fn read_runtime_active_time() -> String {
    let dmi = "/sys/class/dmi/id/power/runtime_active_time";

    file::return_pathdata(dmi)
}

/// Read runtime_status in milliseconds
pub fn read_runtime_status() -> String {
    let dmi = "/sys/class/dmi/id/power/runtime_status";

    file::return_pathdata(dmi)
}

/// Read runtime_suspended_time in seconds
pub fn read_runtime_suspended_time() -> String {
    let dmi = "/sys/class/dmi/id/power/runtime_suspended_time";

    file::return_pathdata(dmi)
}
