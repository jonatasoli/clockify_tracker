use chrono::{DateTime, Utc};
use clap::Parser;
use config::Config;
use std::collections::HashMap;

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
    use chrono::{DateTime, Utc, Local, SecondsFormat};
    use reqwest;
    use reqwest::header;
    use std::fmt::Display;
    use std::collections::HashMap;
    pub async fn start_track<Tz: chrono::TimeZone>(start_date: DateTime<Tz>, config_local: HashMap<String, String>) -> Result<(), reqwest::Error>
    where
        Tz: chrono::TimeZone,
        Tz::Offset: Display,
    {
        let workspace_id: String  = match config_local.get("WORKSPACE_ID") {
            Some(workspace) => workspace.to_string(),
            None => panic!("Workspace ID not found!")
        };
        let project_id: String  = match config_local.get("PROJECT_ID") {
            Some(project_id) => project_id.to_string(),
            None => panic!("PROJECT ID not found!")
        };
        let base_url: String = match config_local.get("BASE_URL"){
            Some(base_url) => base_url.to_string(),
            None => panic!("Base URL not found!")
        };
        let user_id: String = match config_local.get("USER_ID"){
            Some(user_id) => user_id.to_string(),
            None => panic!("User ID not found!")
        };
        let url = format!("{}/workspaces/{}/time-entries", base_url, workspace_id);
        let api_key: String = match config_local.get("API_KEY") {
            Some(api_key) => api_key.to_string(),
            None => panic!("API KEY not found!")
        };
        let mut map = HashMap::new();
        println!("UTC now is: {}", start_date.format("%Y-%m-%dT%TZ"));
        println!("URL: {}", url);
        println!("{} {} {}", workspace_id, base_url, user_id);
        let start = format!("{}", start_date.format("%Y-%m-%dT%TZ"));
        map.insert("start", start);
        map.insert("description", "Working tasks".to_string());
        map.insert("projectID", project_id);
        let mut headers = header::HeaderMap::new();
        headers.insert("content-type", header::HeaderValue::from_static("application/json"));
        headers.insert("X-Api-Key", header::HeaderValue::from_static(&api_key));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_millis(5000))
            .build()?;
        let res = client.post(url)
            .json(&map)
            .send()
            .await?
            .text()
            .await?;
        println!("Start Response = {:?}", res);

        return Ok(());
    }
    pub async fn stop_track<Tz: chrono::TimeZone>(stop_date: DateTime<Tz>, config_local: HashMap<String, String>) -> Result<(), reqwest::Error>
    where
        Tz: chrono::TimeZone,
        Tz::Offset: Display,
    {
        let workspace_id: String  = match config_local.get("WORKSPACE_ID") {
            Some(workspace) => workspace.to_string(),
            None => panic!("Workspace ID not found!")
        };
        let base_url: String = match config_local.get("BASE_URL"){
            Some(base_url) => base_url.to_string(),
            None => panic!("Base URL not found!")
        };
        let user_id: String = match config_local.get("USER_ID"){
            Some(user_id) => user_id.to_string(),
            None => panic!("User ID not found!")
        };
        let url = format!("{}/workspaces/{}/user/{}/time-entries", base_url, workspace_id, user_id);
        let api_key: String = match config_local.get("API_KEY") {
            Some(api_key) => api_key.to_string(),
            None => panic!("API KEY not found!")
        };
        let mut map = HashMap::new();
        println!("UTC now is: {}", stop_date.format("%Y-%m-%dT%TZ"));
        println!("URL: {}", url);
        println!("{} {} {}", workspace_id, base_url, user_id);
        let end = format!("{}", stop_date.format("%Y-%m-%dT%TZ"));
        map.insert("end", end);
        let mut headers = header::HeaderMap::new();
        headers.insert("content-type", header::HeaderValue::from_static("application/json"));
        headers.insert("X-Api-Key", header::HeaderValue::from_static(&api_key));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_millis(5000))
            .build()?;
        let res = client.patch(url)
            .json(&map)
            .send()
            .await?
            .text()
            .await?;
        println!("Stop Response = {:?}", res);

        return Ok(());

    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
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
    println!("{:?}", config_local);
    println!("{:?}", args.track_mode);
    match &*args.track_mode {
        "start" => tracker::start_track(Utc::now(), config_local).await,
        "stop" => tracker::stop_track(Utc::now(), config_local).await,
        _ => panic!("Option is not valid!"),
    }
}
