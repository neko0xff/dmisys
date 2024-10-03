use std::fs;

/*讀取路徑檔案 */
pub fn return_pathdata(path:&str) -> String {
    let data = fs::read_to_string(path)
        .unwrap_or_else(|_| "Unknown".to_string());
    let output = format!("{}",data.trim());

    output
}

/*讀取設定檔*/
pub fn read_config_info(file:&str) -> String {
    if let Ok(contents) = fs::read_to_string(file) {
        return contents;
    }

    "Unknown".to_string()
}

pub fn read_config_var_string(file:&str,find:&str) -> String {
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

pub fn read_config_var_bool(file:&str,find:&str) -> bool {
    if let Ok(contents) = fs::read_to_string(file) {
        for line in contents.lines() {
            if line.starts_with(find) {
                let num = line.trim_start_matches(find).replace("\"", "");
                if num == "1" {
                    return true;
                }else if num == "0"{
                    return false;
                }
            }
        }
    }

    false
}

pub fn read_config_var_usize(file:&str,find:&str) -> usize {
    if let Ok(contents) = fs::read_to_string(file) {
        for line in contents.lines() {
            if line.starts_with(find) {
                let num = line.trim_start_matches(find).replace("\"", "");
                match num.parse::<usize>() {
                    Ok(n) => {
                        return n;
                    },
                    Err(_e) => {
                        return 0;
                    },
                }
            }
        }
    }

    0
}