use std::process::Command;

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
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )
    }
}

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
                // .args(&["-hwaccel", "videotoolbox"])
                .args(&["-i", &args.video])
                .args(&["-ss", &sub.start_time.ffmpeg_format()])
                .args(&["-to", &sub.end_time.ffmpeg_format()])
                .args(args.ffmpeg_opts.split(" "))
                .arg(format!(
                    "{}-{}-{}.{}",
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
