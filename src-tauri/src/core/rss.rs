use rss::Channel;
use reqwest;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://feeds.feedburner.com/rsscna/intworld")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
pub async fn getFeedByUrl(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn getFeedFromFile(path: String) -> Result<Channel, Box<dyn Error>>  {
    let file = File::open(path).unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    Ok(channel)
}

