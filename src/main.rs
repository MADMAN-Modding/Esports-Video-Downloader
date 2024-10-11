use std::io::Cursor;


include!("buffer.rs");
 
#[tokio::main]

async fn main() {
    let channel_id:&str = "UChJJOMnTd2q296Ccd1LG7WQ";

    let xml_url:&str = &format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}");

    let xml = buffer(xml_url);


    // fetch_url(xml_url.to_string(), "feeds.xml".to_string()).await.unwrap();
}
 
// async fn fetch_url(url: String, file_name: String) -> Result<()> {
//     let response = reqwest::get(url).await?;
//     let mut file = std::fs::File::create(file_name)?;
//     let mut content =  Cursor::new(response.bytes().await?);
//     std::io::copy(&mut content, &mut file)?;
//     Ok(())
// }
