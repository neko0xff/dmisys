use colored::*;
use dmisys::*;

pub fn output_msg() {
    println!("\n{}", "Playing Metadata".green().bold());
    println!("{}", "=================".green());
    println!(
        "{:<15} {}",
        "Trackid:".blue().bold(),
        audio::read_audio_trackid()
    );
    println!(
        "{:<15} {}",
        "Title:".blue().bold(),
        audio::read_audio_title()
    );
    println!(
        "{:<15} {}",
        "Source URL:".blue().bold(),
        audio::read_audio_sourceurl()
    );
    println!(
        "{:<15} {}",
        "Album:".blue().bold(),
        audio::read_audio_album()
    );
    println!(
        "{:<15} {}",
        "Artist:".blue().bold(),
        audio::read_audio_artist()
    );
}
