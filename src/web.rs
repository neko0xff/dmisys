use isahc::prelude::*;
use std::error::Error;

/// Get response Data from HTTP WebServer
pub fn cn_server_get(server_url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let http_get = isahc::get(server_url);

    match http_get {
        Ok(mut response) => {
            if response.status().is_success() {
                let get_data = response.text()?;
                Ok(Some(get_data))
            } else {
                Ok(None)
            }
        }
        Err(_e) => Ok(None),
    }
}
