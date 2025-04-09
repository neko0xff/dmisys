use crate::file;

/// Read: the maximum sleep time (in milliseconds) for the system to suspend
/// This is the maximum time the system will wait before suspending
/// when it is idle. The value is read from the `/sys/class/dmi/id/power/autosuspend_delay_ms` file.
/// The value is returned as a `String`.
pub fn read_autosuspend_delay_ms() -> String {
    file::read_dmi_path("power/autosuspend_delay_ms")
}

/// Read: the system's power management mode
/// This indicates whether the system is in a low-power state or not.
/// The value is read from the `/sys/class/dmi/id/power/control` file.
/// The value is returned as a `String`.
pub fn read_control() -> String {
    file::read_dmi_path("power/control")
}

/// Read: runtime_active_time in seconds
/// This indicates the total time the system has been active.
/// The value is read from the `/sys/class/dmi/id/power/runtime_active_time` file.
/// The value is returned as a `String`.
/// The value is in seconds.
pub fn read_runtime_active_time() -> String {
    file::read_dmi_path("power/runtime_active_time")
}

/// Read: runtime_status in milliseconds
/// This indicates the current status of the system's runtime.
/// The value is read from the `/sys/class/dmi/id/power/runtime_status` file.
/// The value is returned as a `String`.
pub fn read_runtime_status() -> String {
    file::read_dmi_path("power/runtime_status")
}

/// Read: runtime_suspended_time in seconds
/// This indicates the total time the system has been suspended.
/// The value is read from the `/sys/class/dmi/id/power/runtime_suspended_time` file.
/// The value is returned as a `String`.
pub fn read_runtime_suspended_time() -> String {
    file::read_dmi_path("power/runtime_suspended_time")
}
