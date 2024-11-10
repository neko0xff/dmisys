use colored::*;
use dmisys::*;

pub fn output_msg() {
    println!("\n{}", "Dev Tools".green().bold());
    println!("{}", "=================".green());
    println!("{}", "Rust".blue().bold());
    println!(
        "{:<15} {}",
        "rustc (compiler):".blue().bold(),
        devtools::read_ver_rustc()
    );
    println!(
        "{:<15} {}",
        "cargo (package manager):".blue().bold(),
        devtools::read_ver_cargo()
    );
    println!(
        "{:<15} {}",
        "rustfmt (code formatter)".blue().bold(),
        devtools::read_ver_rustfmt()
    );
    println!(
        "{:<15} {}",
        "clippy (linter):".blue().bold(),
        devtools::read_ver_clippy()
    );
    println!(
        "{:<15} {}",
        "rustup (toolchain manager)".blue().bold(),
        devtools::read_ver_rustup()
    );
}
