use isahc::prelude::*;
use std::error::Error;

/// Get response Data from HTTP WebServer
/// This function sends an HTTP GET request to the specified server URL.
/// It returns the response data as a `String` if the request is successful.
/// If the request fails or the response status is not successful, it returns `None`.
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
