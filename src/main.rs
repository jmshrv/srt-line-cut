use std::{process::Command, str};

use clap::Parser;
use itertools::Itertools;
use srtlib::{ParsingError, Subtitles, Timestamp};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    video: String,

    #[arg(long)]
    srt: String,

    #[arg(long)]
    phrase: Vec<String>,

    #[arg(long, default_value = "-c:v libx264 -crf 0 -c:a aac -b:a 320k")]
    ffmpeg_opts: String,

    #[arg(long, default_value = "mp4")]
    output_container: String,

    #[arg(long, default_value_t = 16)]
    ffmpeg_instances: usize,
}

trait FFMpegFormat {
    fn ffmpeg_format(&self) -> String;
}

impl FFMpegFormat for Timestamp {
    fn ffmpeg_format(&self) -> String {
        let (hours, minutes, seconds, milliseconds) = self.get();
        format!("{}:{}:{}.{}", hours, minutes, seconds, milliseconds)
    }
}

// trait SecondsBetween {
//     fn seconds_between(&self, other: &Timestamp) -> usize;
// }

// impl SecondsBetween for Timestamp {
//     fn seconds_between(&self, other: &Timestamp) -> usize{
//         let (self_hours, self_minutes, self_seconds, self_milliseconds) = self.get();
//         let (other_hours, other_minutes, other_seconds, other_milliseconds) = other.get();

//         self_hours * 3600 + other_hours * 3600 + self_minutes * 60 + other_minutes * 60 + self_seconds + other_seconds + self_milliseconds / 1000 + other_milliseconds / 1000
//     }
// }

// Apparently ParsingError contains all the errors we have in the program lol
fn main() -> Result<(), ParsingError> {
    let args = Args::parse();
    let subs = Subtitles::parse_from_file(args.srt, None)?;

    for sub in subs {
        let matches: Vec<_> = args
            .phrase
            .iter()
            .filter(|phrase| sub.text.to_lowercase().contains(&phrase.to_lowercase()))
            .collect();

        if !matches.is_empty() {
            Command::new("ffmpeg")
                .args(&["-i", &args.video])
                .args(&["-ss", &sub.start_time.ffmpeg_format()])
                .args(&["-to", &sub.end_time.ffmpeg_format()])
                .args(args.ffmpeg_opts.split(" "))
                .arg(format!(
                    "{:02}-{:02}-{:02}.{:03}",
                    &sub.start_time.ffmpeg_format().replace(":", "_"),
                    &sub.end_time.ffmpeg_format().replace(":", "_"),
                    matches.iter().join(","),
                    &args.output_container
                ))
                .status()?;
        }
    }

    Ok(())
}
