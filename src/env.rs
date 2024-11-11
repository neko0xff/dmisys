use std::{env, process::Command};

/// Read Command: Use a Shell Version
pub fn read_cmd_shellver() -> String {
    let input = read_env_shell();

    let output = match input.as_str() {
        "/usr/bin/zsh" => Command::new("zsh")
            .arg("--version")
            .output()
            .expect("Failed"),
        "/usr/bin/bash" => Command::new("bash")
            .arg("--version")
            .output()
            .expect("Failed"),
        "/usr/bin/fish" => Command::new("fish")
            .arg("--version")
            .output()
            .expect("Failed"),
        _ => return "Failed".to_string(),
    };

    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .nth(1)
        .unwrap_or("Unknown")
        .to_string()
}

/// Use a Shell Version
pub fn read_env_shellver() -> String {
    

    read_cmd_shellver()
}

/// read now Environment value
pub fn read_env_var(key: String) -> String {
    let output: String;
    let key_var = env::var(&key);

    if let Ok(value) = key_var {
        output = value;
    } else {
        output = "Unknown".to_string();
    }

    output
}

/// User Home Directory
pub fn read_env_homedirectory() -> String {
    let find = "HOME";
    

    read_env_var(find.to_string())
}

/// Login User name
pub fn read_env_loginuser() -> String {
    let find = "LOGNAME";
    

    read_env_var(find.to_string())
}

/// User name
pub fn read_env_user() -> String {
    let find = "USER";
    

    read_env_var(find.to_string())
}

/// Use a Shell
pub fn read_env_shell() -> String {
    let find = "SHELL";
    

    read_env_var(find.to_string())
}

/// Language
pub fn read_env_lang() -> String {
    let find = "LANG";
    

    read_env_var(find.to_string())
}

/// Login Session
pub fn read_env_desktopsession() -> String {
    let find = "DESKTOP_SESSION";
    

    read_env_var(find.to_string())
}

/// Display Server
pub fn read_env_displayserver() -> String {
    let find = "XDG_SESSION_TYPE";
    

    read_env_var(find.to_string())
}

/// Desktop Environment
pub fn read_env_displayde_session() -> String {
    let find = "XDG_SESSION_DESKTOP";
    

    read_env_var(find.to_string())
}

/// Desktop Environment
pub fn read_env_displayde_current() -> String {
    let find = "XDG_CURRENT_DESKTOP";
    

    read_env_var(find.to_string())
}

/// Input Method
pub fn read_env_inputmethod() -> String {
    let find = "XMODIFIERS=@im";
    

    read_env_var(find.to_string())
}

///  Working directory: now
pub fn read_env_nowpwd() -> String {
    let find = "PWD";
    

    read_env_var(find.to_string())
}

///  Working directory: old
pub fn read_env_oldpwd() -> String {
    let find = "OLDPWD";
    

    read_env_var(find.to_string())
}

/// Chrome Path
pub fn read_env_chromepath() -> String {
    let find = "CHROME_EXECUTABLE";
    

    read_env_var(find.to_string())
}
