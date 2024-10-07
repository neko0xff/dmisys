use std::error::Error;
use isahc::prelude::*;

/// 向對外的HTTP Server發出: GET
pub fn cn_server_get(server_url: &str) -> Result<Option<String>, Box<dyn Error>> {
    match isahc::get(server_url) {
        Ok(mut response) => {
            if response.status().is_success() {
                let get_data = response.text()?;
                Ok(Some(get_data))
            } else {
                Ok(None)
            }
        }
        Err(_e) => {
            Ok(None) 
        }
    }
}

