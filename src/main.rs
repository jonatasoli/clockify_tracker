use chrono::{DateTime, Utc};
use config::Config;
use std::collections::HashMap;

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
    fn start_track<UTC: chrono::TimeZone>(start_date: DateTime<UTC>) -> String {
        String::from("Stop Date")
    }
    fn stop_track<UTC: chrono::TimeZone>(stop_date: DateTime<UTC>) -> String {
        // let client = reqwest::Client::new();
        // let res = client
        //     .post("http://httpbin.org/post")
        //     .body("the exact body that is sent")
        //     .send()
        //     .await?;
        String::from("Start Date")
    }
}

fn main() {
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
    println!("{:?}", config_local.get("BASE_URL"))
}
