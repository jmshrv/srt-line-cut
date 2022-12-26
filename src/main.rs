use std::str;

use async_process::Command;
use clap::Parser;
use futures::StreamExt;
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

#[tokio::main]
async fn main() -> Result<(), ParsingError> {
    let args = Args::parse();
    let subs = Subtitles::parse_from_file(args.srt, None)?;

    // let mut ffmpeg_futures = Vec::new();

    for sub in subs {
        let matches: Vec<_> = args
            .phrase
            .iter()
            .filter(|phrase| sub.text.to_lowercase().contains(&phrase.to_lowercase()))
            .collect();

        if !matches.is_empty() {
            let command = Command::new("ffmpeg")
                .args(&["-i", &args.video])
                .args(&["-ss", &sub.start_time.ffmpeg_format()])
                .args(&["-to", &sub.end_time.ffmpeg_format()])
                .args(args.ffmpeg_opts.split(" "))
                .arg(format!(
                    "file:{:02}-{:02}-{:02}.{:03}",
                    &sub.start_time.ffmpeg_format(),
                    &sub.end_time.ffmpeg_format(),
                    matches.iter().join(","),
                    &args.output_container
                ))
                .output()
                .await;

            // ffmpeg_futures.push(command);
        }
    }

    // let ffmpeg_stream =
    //     futures::stream::iter(ffmpeg_futures).buffer_unordered(args.ffmpeg_instances);

    // let results = ffmpeg_stream.collect::<Vec<_>>().await;

    // println!("Exported {} segments", results.len());

    Ok(())
}
