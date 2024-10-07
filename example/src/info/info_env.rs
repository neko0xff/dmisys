use dmisys::*;
use colored::*;

pub fn output_msg(){
    println!("\n{}", "System Environment".green().bold());
    println!("{}", "=================".green());
    println!("\n{}","User Info".blue().bold());
    //println!("{:<15} {}", "User:".blue().bold(), env::read_env_user());
    println!("{:<15} {}", "Login User:".blue().bold(), env::read_env_loginuser());
    println!("{:<15} {}", "Shell:".blue().bold(), env::read_env_shell());
    println!("{:<15} {}", "Shell Version:".blue().bold(), env::read_env_shellver());
    
    println!("\n{}","Directory Info".blue().bold());
    println!("{:<15} {}", "Home Directory:".blue().bold(), env::read_env_homedirectory());
    println!("{:<15} {}", "Now Working Directory:".blue().bold(), env::read_env_nowpwd());
    println!("{:<15} {}", "Old Working Directory:".blue().bold(), env::read_env_oldpwd());
    
}