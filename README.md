# SRT Line Cut

This program searches for given segments in an SRT and saves any matching lines as a separate file. **Requires ffmpeg**.

# Usage

```
Usage: srt-line-cut [OPTIONS] --video <VIDEO> --srt <SRT>

Options:
      --video <VIDEO>                        
      --srt <SRT>                            
      --phrase <PHRASE>                      
      --ffmpeg-opts <FFMPEG_OPTS>            [default: "-c:v libx264 -crf 0 -c:a aac -b:a 320k"]
      --output-container <OUTPUT_CONTAINER>  [default: mp4]
      --ffmpeg-instances <FFMPEG_INSTANCES>  [default: 16]
  -h, --help                                 Print help information
  -V, --version                              Print version information
  ```

You can supply multiple phrases, for example `--phrase foo --phrase bar`. Files will be outputted with a to-from timestamp and any matches that were found. By default, the program will output lossless H264 with 320k AAC, which is horribly imbalanced but with MP4 you can't really supply lossless audio.