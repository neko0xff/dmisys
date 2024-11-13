use dmisys::*;
use colored::*;

fn read_ver_cargo() -> &'static str {
    let cargo_ver = env!("CARGO_PKG_VERSION");

    cargo_ver
}

fn output_ver_project(){
    let output = read_ver_cargo();
    println!("Demo Exaple: {}",output);
}

fn output_ver_dmisys(){
    let output = env::read_env_dmisys();
    println!("dmisys Libary: {}",output);
}

pub fn output_msg(){
    println!("\n{}", "Current Use Version".green().bold());
    println!("{}", "===================".green());
    output_ver_project();
    output_ver_dmisys();
}