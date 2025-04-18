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

fn var_cv_string(value:&str) -> String {
    match run_playerctl_command(value) {
        Ok(output) => output,
        Err(_) => "Not Found".to_string()
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
/// This function retrieves the track ID from the output of the `playerctl` command.
pub fn read_audio_trackid() -> String {
    var_cv_string("{{mpris:trackid}}")
}

/// Audio: Track number
/// It uses a regular expression to extract the track ID from the output.
pub fn read_audio_tracknum() -> String {
    var_cv_string("{{xesam:tracknumber}}")
}

/// Audio: Title
/// This function retrieves the title of the currently playing audio track.
pub fn read_audio_title() -> String {
    var_cv_string("{{xesam:title}}")
}

/// Audio: Source URL
/// This function retrieves the source URL of the currently playing audio track.
pub fn read_audio_sourceurl() ->  String {
    var_cv_string("{{xesam:url}}")
}

/// Audio: Album
/// This function retrieves the album name of the currently playing audio track.
pub fn read_audio_album() ->  String {
    var_cv_string("{{xesam:album}}")
}

/// Audio: Artist
/// This function retrieves the artist name of the currently playing audio track.
pub fn read_audio_artist() ->  String {
    var_cv_string("{{xesam:artist}}")
}

/// Audio: Genre
/// This function retrieves the genre of the currently playing audio track.
pub fn read_audio_genre() ->  String {
    var_cv_string("{{xesam:genre}}")
}

/// Audio: Content Created
/// This function retrieves the content creation date of the currently playing audio track.
pub fn read_audio_content_created() ->  String {
    var_cv_string("{{xesam:contentCreated}}")
}

/// Audio: Album Image URL
/// This function retrieves the album image URL of the currently playing audio track.
pub fn read_audio_art_url() ->  String {
    var_cv_string("{{mpris:artUrl}}")
}

/// Audio: Length Time
/// This function retrieves the length of the currently playing audio track.
pub fn read_audio_length() -> String {
    match cal_audio_length() {
        Ok(length) => length,
        Err(_) => "Failed to parse length".to_string()
    }
}
