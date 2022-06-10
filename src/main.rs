use chrono::{DateTime, Local, Utc};
use clap::{App, Arg, ArgEnum, Parser, SubCommand};
use config::Config;
use std::collections::HashMap;
use std::fmt;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Clockify CLI for terminal")]
pub struct Cli {
    #[clap(short, long)]
    track_mode: String,
}

#[allow(unused)]
pub struct StartTracker {
    start: DateTime<Utc>,
    description: String,
    project_id: String,
    billable: bool,
    task_id: String,
    tag_ids: Vec<String>,
}

#[allow(unused)]
pub struct EndTracker {
    end: DateTime<Utc>,
}

#[allow(unused)]
mod tracker {
    use chrono::{DateTime, Utc};
    pub fn start_track<UTC: chrono::TimeZone>(start_date: DateTime<UTC>) {
        println!("Start Date: ");
    }
    pub fn stop_track<UTC: chrono::TimeZone>(stop_date: DateTime<UTC>) {
        // let client = reqwest::Client::new();
        // let res = client
        //     .post("http://httpbin.org/post")
        //     .body("the exact body that is sent")
        //     .send()
        //     .await?;
        println!("Stop Date: ");
    }
}

fn main() {
    // let args = Cli {
    //     mode: track_mode,
    //     time: track_time,
    // };
    let tracker_mode = std::env::args().nth(1).expect("Track Mode");
    let args = Cli::parse();
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let config_local = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    println!("{:?}", config_local.get("BASE_URL"));
    println!("{:?}", args.track_mode);
    match &*args.track_mode {
        "start" => tracker::start_track(Local::now()),
        "stop" => tracker::stop_track(Local::now()),
        _ => panic!("Option is not valid!"),
    }
    ()
}
