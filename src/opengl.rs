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
pub fn read_opengl_version() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL version string: (\d+\.\d+)";
    let version = cv::regex_extract(&output, regex_pattern);

    version
}

/// OpenGL: Vendor
pub fn read_opengl_vendor() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL vendor string: (.*)";
    let vendor = cv::regex_extract(&output, regex_pattern);

    vendor
}

/// OpenGL: Renderer Device
pub fn read_opengl_rendererdevice() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"OpenGL renderer string: (.*)";
    let renderer = cv::regex_extract(&output, regex_pattern);

    renderer
}

/// Renderer: Direct rendering
pub fn read_renderer_direct() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"direct rendering: (.*)";
    let direct = cv::regex_extract(&output, regex_pattern);

    direct
}

/// Renderer: Video RAM
pub fn read_renderer_videoram() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Video memory: (\d+)MB";
    let videoram = cv::regex_extract(&output, regex_pattern);

    videoram
}

/// Renderer: Accelerated
pub fn read_renderer_accelerated() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Accelerated: (.*)";
    let accelerated = cv::regex_extract(&output, regex_pattern);

    accelerated
}

/// Renderer: Share Memory
pub fn read_renderer_sharemenory() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Unified memory: (.*)";
    let sharemenory = cv::regex_extract(&output, regex_pattern);

    sharemenory
}

/// Mesa:  Version
pub fn read_mesa_ver() -> String {
    let output = read_cmd_glxinfo();
    let regex_pattern = r"Version: (.*)";
    let mesa = cv::regex_extract(&output, regex_pattern);

    mesa
}