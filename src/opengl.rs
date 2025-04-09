use crate::cv;
use std::process::Command;

fn read_cmd_glxinfo() -> String {
    let output = Command::new("glxinfo")
        .arg("-B")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// OpenGL: Version
/// This function retrieves the OpenGL version string from the output of the `glxinfo` command.
/// It uses a regular expression to extract the version number from the output.
/// The version string is then returned as a `String`.
pub fn read_opengl_version() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL version string: (\d+\.\d+)";
    let version = cv::regex_extract(&output, regex_pattern);

    version
}

/// OpenGL: Vendor
/// This function retrieves the OpenGL vendor string from the output of the `glxinfo` command.
/// It uses a regular expression to extract the vendor name from the output.
/// The vendor string is then returned as a `String`.
pub fn read_opengl_vendor() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL vendor string: (.*)";
    let vendor = cv::regex_extract(&output, regex_pattern);

    vendor
}

/// OpenGL: Renderer Device
/// This function retrieves the OpenGL renderer device string from the output of the `glxinfo` command.
/// It uses a regular expression to extract the renderer device name from the output.
/// The renderer device string is then returned as a `String`.
pub fn read_opengl_rendererdevice() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL renderer string: (.*)";
    let renderer = cv::regex_extract(&output, regex_pattern);

    renderer
}

/// Renderer: Direct rendering
/// This function retrieves the direct rendering status from the output of the `glxinfo` command.
/// It uses a regular expression to extract the direct rendering status from the output.
/// The status string is then returned as a `String`.
pub fn read_renderer_direct() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"direct rendering: (.*)";
    let direct = cv::regex_extract(&output, regex_pattern);

    direct
}

/// Renderer: Video RAM
/// This function retrieves the video RAM size from the output of the `glxinfo` command.
/// It uses a regular expression to extract the video RAM size from the output.
/// The video RAM size string is then returned as a `String`.
/// The size is typically represented in megabytes (MB).
pub fn read_renderer_videoram() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Video memory: (\d+)MB";
    let videoram = cv::regex_extract(&output, regex_pattern);

    videoram
}

/// Renderer: Accelerated
/// This function retrieves the accelerated rendering status from the output of the `glxinfo` command.
/// It uses a regular expression to extract the accelerated status from the output.
/// The status string is then returned as a `String`.
pub fn read_renderer_accelerated() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Accelerated: (.*)";
    let accelerated = cv::regex_extract(&output, regex_pattern);

    accelerated
}

/// Renderer: Share Memory
/// This function retrieves the share memory status from the output of the `glxinfo` command.
/// It uses a regular expression to extract the share memory status from the output.
/// The status string is then returned as a `String`.
pub fn read_renderer_sharemenory() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Unified memory: (.*)";
    let sharemenory = cv::regex_extract(&output, regex_pattern);

    sharemenory
}

/// Mesa:  Version
/// This function retrieves the Mesa version string from the output of the `glxinfo` command.
/// It uses a regular expression to extract the version number from the output.
/// The Mesa version string is then returned as a `String`.
pub fn read_mesa_ver() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Version: (.*)";
    let mesa = cv::regex_extract(&output, regex_pattern);

    mesa
}