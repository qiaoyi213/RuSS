use reqwest;
use rss::Channel;
use rss::ChannelBuilder;
use rss::Item;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

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
