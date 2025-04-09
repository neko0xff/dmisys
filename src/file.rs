use std::{
    fs, 
    io,
    path::Path
};

/// check this directory is file
/// This function checks if the specified path exists and is a file.
/// It returns `true` if the path is a file and `false` otherwise.
/// If the path does not exist or an error occurs, it returns `false`.
pub fn check_used_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(_m) => {
            return true;
        }
        Err(_e) => {
            return false;
        }   
    }
}

/// check this file type
/// This function checks the type of a file or directory at the specified path.
/// It returns a `Result` containing the type as a `String` if successful.
/// If the path does not exist or an error occurs, it returns an `io::Error`.
/// The function checks if the path is a file, directory, symbolic link, or unknown type.
pub fn check_used_type(path: &str) -> Result<String, io::Error> {
    let metadata = fs::metadata(path)?;
    let file_type = metadata.file_type();

    if file_type.is_file() {
        Ok("File".to_string())
    } else if file_type.is_dir() {
        Ok("Directory".to_string())
    } else if file_type.is_symlink() {
        Ok("SymbolicLink".to_string())
    } else {
        Ok("Unknown".to_string())
    }
}

/// check this directory is kind
/// This function checks if the specified directory exists and is a directory.
/// It returns a `Result` containing a vector of strings representing the entries in the directory.
pub fn check_directory(dir_path: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(dir_path);
    let mut entries = Vec::new();

    if !path.exists() || !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unkown".to_string(),
        ));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        entries.push(file_name.to_string_lossy().into_owned());
    }

    Ok(entries)
}

/// check directory is null
/// This function checks if the specified directory is empty.
/// It returns `true` if the directory is empty and `false` if it contains any entries.
/// If the directory does not exist or an error occurs, it returns `false`.
pub fn check_directory_null(dir_path: &str) -> bool {
    let dir = check_directory(dir_path);

    match dir {
        Ok(entries) => entries.is_empty(),
        Err(_e) => false,
    }
}

/// read & return path a file data
/// This function reads the contents of a file at the specified path.
pub fn return_pathdata(path: &str) -> String {
    let data = fs::read_to_string(path);

    match data {
        Ok(content) => content.trim().to_string(),
        Err(_) => "Unknown".to_string()
    }
}

/// read config file infomation
/// This function reads the contents of a configuration file at the specified path.
pub fn read_config_info(file: &str) -> String {
    let config = fs::read_to_string(file);

    match config {
        Ok(contents) => contents,
        Err(_) => "Unknown".to_string(),
    }
}

/// read power device
/// This function constructs a path to a power supply device's uevent file.
/// It takes the device name (e.g., "BAT" or "ADP") and a number as input.
/// The number is converted to a string and appended to the device name.
/// The resulting path is returned as a `String`.
pub fn read_power_path(device: &str, number: u8) -> String {
    let path = format!("/sys/class/power_supply/{}{}/uevent", device, number.to_string());

    path
}

/// read dmi path
/// This function constructs a path to a DMI (Desktop Management Interface) file.
/// It takes a value (e.g., "bios_version") as input.
/// The value is used to create the path by appending it to the base path `/sys/class/dmi/id/`.
/// The resulting path is returned as a `String`.
/// The function also checks if the file exists and returns "Unknown" if it doesn't.
pub fn read_dmi_path(value: &str) -> String {
    let dmi = format!("/sys/class/dmi/id/{}", value);

    return_pathdata(&dmi)
}

/// read config file a value(String)
/// This function reads a configuration file and extracts a specific value based on the provided key.
/// It takes the file path and the key as input.
/// It searches for the key in the file's contents and returns the corresponding value as a `String`.
/// If the key is not found, it returns "Unknown".
pub fn read_config_var_string(file: &str, find: &str) -> String {
    if let Ok(contents) = fs::read_to_string(file) {
        for line in contents.lines() {
            if line.starts_with(find) {
                let data = line.trim_start_matches(find).replace("\"", "").to_string();
                return data;
            }
        }
    }

    "Unknown".to_string()
}

/// read config file a value(bool)
/// This function reads a configuration file and extracts a specific boolean value based on the provided key.
/// It takes the file path and the key as input.
/// It searches for the key in the file's contents and returns the corresponding value as a `bool`.
/// If the key is not found, it returns `false`.
/// The function expects the value to be either "1" (true) or "0" (false).
/// If the value is not in this format, it defaults to `false`.
pub fn read_config_var_bool(file: &str, find: &str) -> bool {
    if let Ok(contents) = fs::read_to_string(file) {
        for line in contents.lines() {
            if line.starts_with(find) {
                let num = line.trim_start_matches(find).replace("\"", "");
                if num == "1" {
                    return true;
                } else if num == "0" {
                    return false;
                }
            }
        }
    }

    false
}

/// read config file a value(usize)
/// This function reads a configuration file and extracts a specific value based on the provided key.
/// It takes the file path and the key as input.
/// It searches for the key in the file's contents and returns the corresponding value as a `usize`.
/// If the key is not found or if the value cannot be parsed as a `usize`, it returns `0`.
/// The function expects the value to be in a format that can be parsed as a `usize`.
/// If the value is not in this format, it defaults to `0`.
pub fn read_config_var_usize(file: &str, find: &str) -> usize {
    if let Ok(contents) = fs::read_to_string(file) {
        for line in contents.lines() {
            if line.starts_with(find) {
                let num = line.trim_start_matches(find).replace("\"", "");
                match num.parse::<usize>() {
                    Ok(n) => {
                        return n;
                    }
                    Err(_e) => {
                        return 0;
                    }
                }
            }
        }
    }

    0
}
