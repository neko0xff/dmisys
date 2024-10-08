use std::{
    env,
    process::Command
};

/// Read Command: Use a Shell Version
fn read_cmd_shellver() ->String{
    let input = read_env_shell();
    
    let output = match input.as_str() {
        "/usr/bin/zsh" => {
            Command::new("zsh")
                .arg("--version")
                .output()
                .expect("Failed")
        },
        "/usr/bin/bash" => {
            Command::new("bash")
                .arg("--version")
                .output()
                .expect("Failed")
        },
        "/usr/bin/fish" => {
            Command::new("fish")
                    .arg("--version")
                    .output()
                    .expect("Failed")
        },
        _ => {
            return "Failed".to_string()
        }
    };
    
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .nth(1)
        .unwrap_or("Unknown")
        .to_string()
}

/// Use a Shell Version
pub fn read_env_shellver() -> String {
    let output = read_cmd_shellver();

    output
}

/// read now Environment value
pub fn read_env_var(key: String) -> String {
    let output: String;
    let key_var = env::var(&key);

    if let Ok(value) = key_var {
        output = value;
    }else{
        output = "Unknown".to_string();
    }

    output
}

/// User Home Directory
pub fn read_env_homedirectory() -> String {
    let find = "HOME";
    let output = read_env_var(find.to_string());

    output
}

/// Login User name
pub fn read_env_loginuser() -> String {
    let find = "LOGNAME";
    let output = read_env_var(find.to_string());

    output
}

/// User name
pub fn read_env_user() -> String {
    let find = "USER";
    let output = read_env_var(find.to_string());

    output
}

/// Use a Shell
pub fn read_env_shell() -> String {
    let find = "SHELL";
    let output = read_env_var(find.to_string());

    output
}

/// Language
pub fn read_env_lang() -> String {
    let find = "LANG";
    let output = read_env_var(find.to_string());

    output
}

/// Login Session
pub fn read_env_desktopsession() -> String {
    let find = "DESKTOP_SESSION";
    let output = read_env_var(find.to_string());

    output
}

/// Display Server
pub fn read_env_displayserver() -> String {
    let find = "XDG_SESSION_TYPE";
    let output = read_env_var(find.to_string());

    output
}

/// Desktop Environment
pub fn read_env_displayde_session() -> String {
    let find = "XDG_SESSION_DESKTOP";
    let output = read_env_var(find.to_string());

    output
}

/// Desktop Environment
pub fn read_env_displayde_current() -> String {
    let find = "XDG_CURRENT_DESKTOP";
    let output = read_env_var(find.to_string());

    output
}

/// Input Method
pub fn read_env_inputmethod() -> String {
    let find = "XMODIFIERS=@im";
    let output = read_env_var(find.to_string());

    output
}

///  Working directory: now
pub fn read_env_nowpwd() -> String {
    let find = "PWD";
    let output = read_env_var(find.to_string());

    output
}

///  Working directory: old
pub fn read_env_oldpwd() -> String {
    let find = "OLDPWD";
    let output = read_env_var(find.to_string());

    output
}

/// Chrome Path
pub fn read_env_chromepath() -> String{
    let find = "CHROME_EXECUTABLE";
    let output = read_env_var(find.to_string());

    output
}
