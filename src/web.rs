use std::error::Error;
use isahc::prelude::*;

pub fn cn_server_get(server_url: &str) -> Result<Option<String>, Box<dyn Error>> {
    match isahc::get(server_url) {
        Ok(mut response) => {
            if response.status().is_success() {
                let public_ip = response.text()?;
                Ok(Some(public_ip))
            } else {
                Ok(None)
            }
        }
        Err(_e) => {
            Ok(None) 
        }
    }
}

