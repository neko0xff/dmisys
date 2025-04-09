use crate::cv;
use std::process::Command;

fn read_cmd_yarn() -> String {
    let output = Command::new("yarn")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_npm() -> String {
    let output = Command::new("npm")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_node() -> String {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_deno() -> String {
    let output = Command::new("deno")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_golang() -> String {
    let output = Command::new("go")
        .arg("version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_firefox() -> String {
    let output = Command::new("firefox")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_rustc() -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_cargo() -> String {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_rustfmt() -> String {
    let output = Command::new("rustfmt")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_clippy() -> String {
    let output = Command::new("clippy-driver")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_rustup() -> String {
    let output = Command::new("rustup")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_jdk() -> String {
    let output = Command::new("java")
        .arg("-version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_qt() -> String {
    let output = Command::new("qmake")
        .args(&["-query", "QT_VERSION"])
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_gtk() -> String {
    let output = Command::new("gtk-launch")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn read_cmd_gtk4() -> String {
    let output = Command::new("gtk4-launch")
        .arg("--version")
        .output()
        .expect("Failed find");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// QT Version
/// This function retrieves the QT version string from the output of the `qmake` command.
/// The version string is then returned as a `String`.
pub fn read_ver_qt() -> String {
    let output = read_cmd_qt();
    let regex_pattern = r"(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// node Version
/// This function retrieves the node version string from the output of the `node` command.
/// The version string is then returned as a `String`.
pub fn read_ver_node() -> String {
    let output = read_cmd_node();
    let regex_pattern = r"v(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// npm Version
/// This function retrieves the npm version string from the output of the `npm` command.
/// The version string is then returned as a `String`.
pub fn read_ver_npm() -> String {
    let output = read_cmd_npm();
    let regex_pattern = r"(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// yarn Version
/// This function retrieves the yarn version string from the output of the `yarn` command.
/// The version string is then returned as a `String`.
pub fn read_ver_yarn() -> String {
    let output = read_cmd_yarn();
    let regex_pattern = r"(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// deno Version
/// This function retrieves the deno version string from the output of the `deno` command.
/// The version string is then returned as a `String`.
pub fn read_ver_deno() -> String {
    let output = read_cmd_deno();
    let regex_pattern = r"(?m)deno (\d+\.\d+\.\d+\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// Go-lang Version
/// This function retrieves the Go-lang version string from the output of the `go` command.
/// The version string is then returned as a `String`.
pub fn read_ver_golang() -> String {
    let output = read_cmd_golang();
    let regex_pattern = r"go(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// Firefox Version
/// This function retrieves the Firefox version string from the output of the `firefox` command.
/// The version string is then returned as a `String`.
pub fn read_ver_firefox() -> String {
    let output = read_cmd_firefox();
    let regex_pattern = r"(?:\d+\.\d+\.\d+(?:[a-zA-Z]\d*|esr|b\d+|rc\d+|alpha\d*|beta\d*)?)";

    cv::regex_extract(&output, regex_pattern)
}

/// JDK Version
/// This function retrieves the JDK version string from the output of the `java` command.
/// The version string is then returned as a `String`.
pub fn read_ver_jdk() -> String {
    let output = read_cmd_jdk();
    let regex_pattern = r"(?:\d+(?:\.\d+){0,2}(?:_\d+)?(?:-ea|-ga|-internal)?(?:\+\d+)?(?:-LTS)?)";

    cv::regex_extract(&output, regex_pattern)
}

/// Rustc compiler Version
/// This function retrieves the Rustc compiler version string from the output of the `rustc` command.
/// The version string is then returned as a `String`.
pub fn read_ver_rustc() -> String {
    let output = read_cmd_rustc();
    let regex_pattern = r"(?m)rustc (\d+\.\d+\.\d+\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// cargo package manager Version
/// This function retrieves the Cargo package manager version string from the output of the `cargo` command.
/// The version string is then returned as a `String`.
pub fn read_ver_cargo() -> String {
    let output = read_cmd_cargo();
    let regex_pattern = r"(?m)cargo (\d+\.\d+\.\d+\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// rustfmt code formatter Version
/// This function retrieves the Rustfmt code formatter version string from the output of the `rustfmt` command.
/// The version string is then returned as a `String`.
pub fn read_ver_rustfmt() -> String {
    let output = read_cmd_rustfmt();
    let regex_pattern = r"(?m)rustfmt (\d+\.\d+\.\d+(?:-(?:stable|nightly|beta))?\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// clippy linter Version
/// This function retrieves the Clippy linter version string from the output of the `clippy-driver` command.
/// The version string is then returned as a `String`.
pub fn read_ver_clippy() -> String {
    let output = read_cmd_clippy();
    let regex_pattern = r"(?m)clippy (\d+\.\d+\.\d+\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// rustup toolchain manager Version
/// This function retrieves the Rustup toolchain manager version string from the output of the `rustup` command.
/// The version string is then returned as a `String`.
pub fn read_ver_rustup() -> String {
    let output = read_cmd_rustup();
    let regex_pattern = r"(?m)rustup (\d+\.\d+\.\d+\s\([^)]+\))";

    cv::regex_extract(&output, regex_pattern)
}

/// GTK Version
/// This function retrieves the GTK version string from the output of the `gtk-launch` command.
/// The version string is then returned as a `String`.
pub fn read_ver_gtk() -> String {
    let output = read_cmd_gtk();
    let regex_pattern = r"(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}

/// GTK 4 Version
/// This function retrieves the GTK 4 version string from the output of the `gtk4-launch` command.
/// The version string is then returned as a `String`.
pub fn read_ver_gtk4() -> String {
    let output = read_cmd_gtk4();
    let regex_pattern = r"(\d+\.\d+\.\d+)";

    cv::regex_extract(&output, regex_pattern)
}
