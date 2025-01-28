use reqwest;
use rss::Channel;
use rss::ChannelBuilder;
use rss::Item;
use serde_json::{json,Value};
use std::error::Error;
use std::io::{Read, Write, BufReader};
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::vec::Vec;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Source {
    title: String,
    description: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SourcesFile {
    sources: Vec<Source>,
    favorites: Vec<Source>,
}

#[tauri::command]
pub fn example_feed(url: String) -> Result<Vec<String>, String> {
    let response = reqwest::blocking::get(url);
    match response {
        Ok(resp) => {
            let body = resp.text().unwrap_or_else(|_| "".to_string());

            let channel = Channel::read_from(body.as_bytes()).map_err(|e| e.to_string())?;
            println!("{}", channel.title().to_string());
            let items = channel.items.clone();
            let mut feeds = Vec::new();

            for item in items.iter() {
                let json = json!({
                    "title": item.title.as_ref().unwrap(),
                    "link": item.link.as_ref().unwrap(),
                    "pubDate": item.pub_date.as_ref().unwrap(),
                    "description": item.description.as_ref().unwrap()
                });
                feeds.push(json.to_string());
                println!("title")
            }
            Ok(feeds)
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
pub fn getSources() -> Result<String, String> {
    let path = Path::new("./sources.json");
    // check sources.json is exists or not
    // if exist, read the json file and parse it to list
    // if do not exist, create a file sources.json and return an empty list

    if path.exists() {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        Ok(contents)         
    } else {
        let new_sources_file = SourcesFile {
            sources: vec![],
            favorites: vec![]
        };

        let json_data = serde_json::to_string_pretty(&new_sources_file).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json_data.as_bytes()).map_err(|e| e.to_string())?;

        Ok(json_data) // 返回空列表
    }
}

pub async fn getFeedByUrl(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn getFeedFromFile(path: String) -> Result<Channel, Box<dyn Error>> {
    let file = File::open(path).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    Ok(channel)
}
