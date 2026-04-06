use feed_rs::model::{Entry, Feed};
use feed_rs::parser::Builder;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    title: String,
    description: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourcesFile {
    sources: Vec<Source>,
}

fn parse_feed(body: &[u8], base_uri: Option<&str>) -> Result<Feed, String> {
    Builder::new()
        .base_uri(base_uri)
        .build()
        .parse(body)
        .map_err(|e| e.to_string())
}

fn entry_title(entry: &Entry) -> String {
    entry
        .title
        .as_ref()
        .map(|t| t.content.clone())
        .unwrap_or_default()
}

fn entry_link(entry: &Entry) -> String {
    if let Some(link) = entry
        .links
        .iter()
        .find(|l| l.rel.as_deref() == Some("alternate") && l.media_type.as_deref() == Some("text/html"))
    {
        return link.href.clone();
    }

    if let Some(link) = entry.links.iter().find(|l| l.rel.as_deref() == Some("alternate")) {
        return link.href.clone();
    }

    if let Some(link) = entry
        .links
        .iter()
        .find(|l| l.rel.is_none() && l.media_type.as_deref() == Some("text/html"))
    {
        return link.href.clone();
    }

    if let Some(link) = entry.links.iter().find(|l| l.rel.is_none()) {
        return link.href.clone();
    }

    entry
        .links
        .first()
        .map(|l| l.href.clone())
        .unwrap_or_default()
}

fn entry_pub_date(entry: &Entry) -> String {
    entry
        .published
        .or(entry.updated)
        .map(|d| d.to_rfc2822())
        .unwrap_or_default()
}

fn entry_description(entry: &Entry) -> String {
    if let Some(summary) = &entry.summary {
        return summary.content.clone();
    }

    if let Some(content) = &entry.content {
        if let Some(body) = &content.body {
            return body.clone();
        }
    }

    String::new()
}

#[tauri::command]
pub fn example_feed(url: String) -> Result<Vec<String>, String> {
    let response = reqwest::blocking::get(&url);
    match response {
        Ok(resp) => {
            let body = resp.bytes().map_err(|e| e.to_string())?;
            let feed = parse_feed(body.as_ref(), Some(&url))?;
            let mut feeds = Vec::new();

            for entry in &feed.entries {
                let json = json!({
                    "title": entry_title(entry),
                    "link": entry_link(entry),
                    "pubDate": entry_pub_date(entry),
                    "description": entry_description(entry)
                });
                feeds.push(json.to_string());
            }
            Ok(feeds)
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
pub fn getFeed(url: String) -> Result<String, String> {
    let response = reqwest::blocking::get(url.clone());
    match response {
        Ok(resp) => {
            Ok(resp.text().unwrap_or_else(|_| "".to_string()))
        }
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn getSourceInfo(url: String) -> Result<String, String> {
    let response = reqwest::blocking::get(url.clone());
    match response {
        Ok(resp) => {
            let body = resp.bytes().map_err(|e| e.to_string())?;
            let feed = parse_feed(body.as_ref(), Some(&url))?;

            let source = Source {
                title: feed
                    .title
                    .as_ref()
                    .map(|t| t.content.clone())
                    .unwrap_or_default(),
                description: feed
                    .description
                    .as_ref()
                    .map(|d| d.content.clone())
                    .unwrap_or_default(),
                link: url.clone()
            };
            let ans = serde_json::to_string_pretty(&source).map_err(|e| e.to_string())?;

            Ok(ans)  
        }
        Err(e) => Err(e.to_string())
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
            sources: vec![]
        };

        let json_data = serde_json::to_string_pretty(&new_sources_file).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json_data.as_bytes()).map_err(|e| e.to_string())?;

        Ok(json_data) 
    }
}
#[tauri::command]
pub fn addSource(title: String, description: String, link: String) -> Result<(), String> {

    let path = Path::new("./sources.json");
    let s = Source {
        title:  title,
        description: description,
        link: link
    };

    if path.exists() {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        let mut json_data: SourcesFile = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

        json_data.sources.push(s);
        let json_data_str = serde_json::to_string_pretty(&json_data).map_err(|e| e.to_string())?;
        let mut file = OpenOptions::new().write(true).truncate(true).open(path).map_err(|e| e.to_string())?;
        file.write_all(json_data_str.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    } else {

        let mut new_sources_file = SourcesFile {
            sources: vec![]
        };

        new_sources_file.sources.push(s);

        let json_data = serde_json::to_string_pretty(&new_sources_file).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json_data.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[tauri::command]
pub fn deleteSource(title: String) -> Result<(), String> {
    let path = Path::new("./sources.json");
    if path.exists() {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        let mut json_data: SourcesFile = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
        // search and delete source
        json_data.sources.retain(|item| if *item.title == title {false} else {true});


        let json_data_str = serde_json::to_string_pretty(&json_data).map_err(|e| e.to_string())?;
        let mut file = OpenOptions::new().write(true).truncate(true).open(path).map_err(|e| e.to_string())?;
        file.write_all(json_data_str.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())

    } else {
        Ok(())
    }
}
