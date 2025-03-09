use colored::*;
use dmisys::*;

pub fn output_msg() {
    println!("\n{}", "Desktop Environment".green().bold());
    println!("{}", "=================".green());

    println!("\n{}", "Session".blue().bold());
    println!(
        "{:<15} {}",
        "Login Session:".blue().bold(),
        env::read_env_desktopsession()
    );
    println!(
        "{:<15} {}",
        "Desktop:".blue().bold(),
        env::read_env_displayde_current()
    );
    println!(
        "{:<15} {}",
        "Display Server:".blue().bold(),
        env::read_env_displayserver()
    );

    println!("\n{}", "Display Screen".blue().bold());
    println!(
        "{:<15} {}",
        "ID:".blue().bold(),
        display::read_display_id()
    );
    println!(
        "{:<15} {}",
        "Resolution:".blue().bold(),
        display::read_display_resolution()
    );

    println!("\n{}", "Internationality".blue().bold());
    println!(
        "{:<15} {}",
        "Input Method:".blue().bold(),
        env::read_env_inputmethod()
    );
    println!("{:<15} {}", "Language:".blue().bold(), env::read_env_lang());
    println!("\n{}", "Display Server".green().bold());
    println!("{}", "=================".green());
    println!(
        "{:<15} {:?}",
        "X Server:".blue().bold(),
        display::read_xserver_ver()
    );
}
