use std::error::Error;
use rss::Channel;

async fn buffer(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    
    Ok(channel)
}