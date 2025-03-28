use std::{
    fs, 
    io,
    path::Path
};

/// check this directory is file
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
pub fn check_directory_null(dir_path: &str) -> bool {
    let dir = check_directory(dir_path);

    match dir {
        Ok(entries) => entries.is_empty(),
        Err(_e) => false,
    }
}

/// read & return path a file data
pub fn return_pathdata(path: &str) -> String {
    let data = fs::read_to_string(path);

    match data {
        Ok(content) => content.trim().to_string(),
        Err(_) => "Unknown".to_string()
    }
}

/// read config file infomation
pub fn read_config_info(file: &str) -> String {
    let config = fs::read_to_string(file);

    match config {
        Ok(contents) => contents,
        Err(_) => "Unknown".to_string(),
    }
}

/// read power device
pub fn read_power_path(device: &str, number: u8) -> String {
    let path = format!("/sys/class/power_supply/{}{}/uevent", device, number.to_string());

    path
}

/// read dmi path
pub fn read_dmi_path(value: &str) -> String {
    let dmi = format!("/sys/class/dmi/id/{}", value);

    return_pathdata(&dmi)
}

/// read config file a value(String)
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
