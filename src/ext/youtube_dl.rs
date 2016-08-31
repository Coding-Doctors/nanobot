use chrono::UTC;
use serde_json;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use ::error::{Error, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct YoutubeDLData {
    pub duration: u64,
    pub fulltitle: String,
    pub title: String,
    pub uploader: String,
    pub view_count: u64,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub data: YoutubeDLData,
    pub filepath: String,
}

pub fn download(url: &str) -> Result<Response> {
    let filepathrel = {
        let utc = UTC::now();

        format!("./mu/{}{}.mp3", utc.timestamp(), utc.timestamp_subsec_nanos())
    };

    // --no-mtime: Set the touch time to now, rather than when the file was
    // uploaded
    //
    // --audio-format: Use mp3. It's lossy and low-quality, and discord only
    // goes up to 128kbps (usually)
    //
    // --output: Specify a file to output to. youtube-dl won't give us
    // data from the output cleanly, so we can use this to search for the
    // resultant info json file and the file itself.
    //
    // --write-info-json: write info about the video to a file, so that it can
    // be later loaded. This is mostly useful for the song name, duration, and
    // perhaps a truncated description. The duration is useful here for denying
    // songs over the song duration limit easily.
    let cmd_res = Command::new("youtube-dl")
        .arg("--no-mtime")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--output")
        .arg(&filepathrel)
        .arg("--write-info-json")
        .arg(&url)
        .output();

    let cmd = match cmd_res {
        Ok(cmd) => cmd,
        Err(why) => {
            warn!("downloading {}: {:?}", url, why);

            return Err(Error::YoutubeDL("Error downloading song".to_owned()));
        },
    };

    if !cmd.status.success() {
        warn!("exit code downloading {}: {:?}", url, cmd.status.code());
        warn!("ytdl stderr: {:?}", cmd.stderr);

        return Err(Error::YoutubeDL("Error downloading song".to_owned()));
    }

    let json_path = format!("{}.info.json", filepathrel);

    let file = match File::open(Path::new(&json_path)) {
        Ok(file) => file,
        Err(why) => {
            warn!("opening {}: {:?}", json_path, why);

            println!("{:?}", why);

            return Err(Error::YoutubeDL("Error getting song info".to_owned()));
        },
    };

    let data: YoutubeDLData = match serde_json::from_reader(file) {
        Ok(data) => data,
        Err(why) => {
            warn!("parsing {}: {:?}", json_path, why);

            return Err(Error::YoutubeDL("Error parsing song info".to_owned()));
        },
    };

    Ok(Response {
        data: data,
        filepath: filepathrel,
    })
}