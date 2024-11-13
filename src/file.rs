use std::path::Path;
use std::{fs, io};

/// check this directory is file
pub fn check_used_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);

    metadata.is_ok()
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
    match check_directory(dir_path) {
        Ok(entries) => entries.is_empty(),
        Err(_e) => false,
    }
}

/// read & return path a file data
pub fn return_pathdata(path: &str) -> String {
    let data = fs::read_to_string(path).unwrap_or_else(|_| "Unknown".to_string());
    let output = data.trim().to_string();

    output
}

/// read config file infomation
pub fn read_config_info(file: &str) -> String {
    if let Ok(contents) = fs::read_to_string(file) {
        return contents;
    }

    "Unknown".to_string()
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
