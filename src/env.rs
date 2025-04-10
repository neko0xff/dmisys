use std::{
    env, 
    process::Command
};

/// Read Command: Use a Shell Version
/// This function retrieves the version of the shell specified in the environment variable `SHELL`.
/// It uses the `Command` struct to execute the shell with the `--version` argument.
/// The output is captured and parsed to extract the version number.
/// The version string is then returned as a `String`.
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
/// This function retrieves the version of the shell specified in the environment variable `SHELL`.
pub fn read_env_shellver() -> String {
    read_cmd_shellver()
}

/// read now Environment value
/// It uses the `read_cmd_shellver` function to execute the shell with the `--version` argument.
/// The output is captured and parsed to extract the version number.
/// The version string is then returned as a `String`.
pub fn read_env_var(key: String) -> String {
    let key_var = env::var(&key);

    match key_var {
        Ok(value) => value,
        Err(_) => "Unknown".to_string()
    }
}

/// User Home Directory
/// This function retrieves the user's home directory from the environment variable `HOME`.
pub fn read_env_homedirectory() -> String {
    let find = "HOME";

    read_env_var(find.to_string())
}

/// Login User name
/// This function retrieves the login user name from the environment variable `LOGNAME`.
pub fn read_env_loginuser() -> String {
    let find = "LOGNAME";

    read_env_var(find.to_string())
}

/// User name
/// This function retrieves the user name from the environment variable `USER`.
pub fn read_env_user() -> String {
    let find = "USER";

    read_env_var(find.to_string())
}

/// Use a Shell
/// This function retrieves the shell used by the user from the environment variable `SHELL`.
pub fn read_env_shell() -> String {
    let find = "SHELL";

    read_env_var(find.to_string())
}

/// Language
/// This function retrieves the language setting from the environment variable `LANG`.
/// It is used to determine the language and locale settings for the user's environment.
/// The value is returned as a `String`.
pub fn read_env_lang() -> String {
    let find = "LANG";

    read_env_var(find.to_string())
}

/// Desktop  Session
/// This function retrieves the desktop session information from the environment variable `DESKTOP_SESSION`.
/// It is used to identify the current desktop environment being used.
/// The value is returned as a `String`.
pub fn read_env_desktopsession() -> String {
    let find = "DESKTOP_SESSION";

    read_env_var(find.to_string())
}

/// Display Server
/// This function retrieves the display server information from the environment variable `XDG_SESSION_TYPE`.
/// It is used to identify the type of display server being used.
/// The value is returned as a `String`.
pub fn read_env_displayserver() -> String {
    let find = "XDG_SESSION_TYPE";

    read_env_var(find.to_string())
}

/// Desktop Environment
/// This function retrieves the desktop environment information from the environment variable `XDG_SESSION_DESKTOP`.
/// It is used to identify the current desktop environment being used.
/// The value is returned as a `String`.
pub fn read_env_displayde_session() -> String {
    let find = "XDG_SESSION_DESKTOP";

    read_env_var(find.to_string())
}

/// Desktop Environment
/// This function retrieves the current desktop environment information from the environment variable `XDG_CURRENT_DESKTOP`.
/// It is used to identify the current desktop environment being used.
/// The value is returned as a `String`.
pub fn read_env_displayde_current() -> String {
    let find = "XDG_CURRENT_DESKTOP";

    read_env_var(find.to_string())
}

/// Input Method
/// This function retrieves the input method information from the environment variable `XMODIFIERS`.
/// It is used to identify the input method framework being used.
/// The value is returned as a `String`.
pub fn read_env_inputmethod() -> String {
    let find = "XMODIFIERS=@im";

    read_env_var(find.to_string())
}

///  Working directory: now
/// This function retrieves the current working directory from the environment variable `PWD`.
/// It is used to identify the current directory in which the user is working.
/// The value is returned as a `String`.
pub fn read_env_nowpwd() -> String {
    let find = "PWD";

    read_env_var(find.to_string())
}

///  Working directory: old
/// This function retrieves the previous working directory from the environment variable `OLDPWD`.
/// It is used to identify the last directory the user was in before changing to the current one.
/// The value is returned as a `String`.
pub fn read_env_oldpwd() -> String {
    let find = "OLDPWD";

    read_env_var(find.to_string())
}

/// Chrome Path
/// This function retrieves the path to the Chrome executable from the environment variable `CHROME_EXECUTABLE`.
/// It is used to identify the location of the Chrome browser executable.
/// The value is returned as a `String`.
/// If the environment variable is not set, it returns "Unknown".
/// The function is used to check if the Chrome browser is installed and accessible.
pub fn read_env_chromepath() -> String {
    let find = "CHROME_EXECUTABLE";

    read_env_var(find.to_string())
}


/// dmisys libary Version
/// This function retrieves the version of the dmisys library from the environment variable `CARGO_PKG_VERSION`.
/// It is used to identify the version of the library being used.
/// The value is returned as a `String`.
pub fn read_env_dmisys() -> String {
    let find = "CARGO_PKG_VERSION";

    match  env::var(find) {
        Ok(value) => value,
        Err(_) => "Unknown".to_string()
    }
}
