use rss::Channel;

pub async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://feeds.feedburner.com/rsscna/intworld")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
