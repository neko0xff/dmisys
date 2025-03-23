use std::process::Command;
use crate::cv;

/// Read Command: playerctl
fn run_playerctl_command(format: &str) -> Result<String, String> {
    let output = Command::new("playerctl")
        .args(&["metadata", "--format", format])
        .output()
        .map_err(|e| format!("Failed to execute playerctl: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if stdout.is_empty() {
        Err("No data received from playerctl".to_string())
    } else {
        Ok(stdout)
    }
}

fn cal_audio_length() -> Result<String, String> {
    match run_playerctl_command("{{mpris:length}}") {
        Ok(length_str) => {
            if let Ok(length_ns) = length_str.parse::<u64>() {
                let size = cv::ns_to_sec(length_ns);
                let (hours, minutes, seconds) = cv::sec_to_playingtime(size);
                Ok(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
            } else {
                Err("Failed to parse length as number".to_string())
            }
        }
        Err(e) => Err(e),
    }
}

/// Audio: Track ID
pub fn read_audio_trackid() -> String {
    match run_playerctl_command("{{mpris:trackid}}") {
        Ok(trackid) => trackid,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Track number
pub fn read_audio_tracknum() -> String {
    match run_playerctl_command("{{xesam:tracknumber}}") {
        Ok(tracknumber) => tracknumber,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Title
pub fn read_audio_title() -> String {
    match run_playerctl_command("{{xesam:title}}") {
        Ok(title) => title,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Source URL
pub fn read_audio_sourceurl() ->  String {
    match run_playerctl_command("{{xesam:url}}") {
        Ok(source) => source,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Album
pub fn read_audio_album() ->  String {
    match run_playerctl_command("{{xesam:album}}") {
        Ok(album) => album,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Artist
pub fn read_audio_artist() ->  String {
    match run_playerctl_command("{{xesam:artist}}") {
        Ok(artist) => artist,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Genre
pub fn read_audio_genre() ->  String {
    match run_playerctl_command("{{xesam:genre}}") {
        Ok(genre) => genre,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Content Created
pub fn read_audio_content_created() ->  String {
    match run_playerctl_command("{{xesam:contentCreated}}") {
        Ok(contentcreated) => contentcreated,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Album Image URL
pub fn read_audio_art_url() ->  String {
    match run_playerctl_command("{{mpris:artUrl}}") {
        Ok(arturl) => arturl,
        Err(_) => "Not Found".to_string()
    }
}

/// Audio: Length Time
pub fn read_audio_length() -> String {
    match cal_audio_length() {
        Ok(length) => length,
        Err(_) => "Failed to parse length".to_string()
    }
}
