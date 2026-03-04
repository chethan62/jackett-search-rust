use std::env;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Rss { channel: Channel }

#[derive(Deserialize, Debug)]
struct Channel { item: Vec<Item> }

#[derive(Deserialize, Debug)]
struct Item {
    title: String,
    link: String,
    #[serde(rename = "size")]
    size: Option<u64>,
    #[serde(rename = "attr", default)]
    attributes: Vec<TorznabAttr>,
}

#[derive(Deserialize, Debug)]
struct TorznabAttr {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { return Ok(()); }

    let api_key = &args[1];
    let query = &args[2];

    let cat_id = if args.len() > 3 {
        match args[3].to_lowercase().as_str() {
            "movies" => "2000",
            "tv" => "5000",
            "music" => "3000",
            "games" => "1000,4000",
            "software" => "4000",
            "books" => "8000",
            _ => "",
        }
    } else { "" };

    let url = format!(
        "http://127.0.0.1:9117/api/v2.0/indexers/all/results/torznab/api?apikey={}&q={}&cat={}",
        api_key, query, cat_id
    );

    let client = reqwest::Client::builder()
    .user_agent("Mozilla/5.0 (X11; Linux x86_64; qBittorrent-Rust-Search)")
    .timeout(Duration::from_secs(45))
    .tcp_keepalive(Duration::from_secs(60))
    .build()?;

    let response = client.get(url).send().await?;
    if !response.status().is_success() { return Ok(()); }

    let body = response.text().await?;
    if !body.trim().starts_with('<') { return Ok(()); }

    let rss: Rss = from_str(&body)?;

    for it in rss.channel.item {
        // Filter: Prioritize magnet links if they exist in attributes
        let magnet_link = it.attributes.iter()
        .find(|a| a.name == "magneturl")
        .map(|a| a.value.clone())
        .unwrap_or_else(|| it.link.clone());

        let seeds = it.attributes.iter().find(|a| a.name == "seeders").map(|a| a.value.as_str()).unwrap_or("0");
        let leech = it.attributes.iter().find(|a| a.name == "peers").map(|a| a.value.as_str()).unwrap_or("0");
        let indexer = it.attributes.iter().find(|a| a.name == "jackettindexer").map(|a| a.value.as_str()).unwrap_or("Jackett");

        let display_name = format!("[{}] {}", indexer, it.title.replace('|', ""));

        // Format: link|name|size|seeds|leech|engine_url|desc_link
        println!("{}|{}|{}|{}|{}|{}|{}",
                 magnet_link, display_name, it.size.unwrap_or(0),
                 seeds, leech, "http://127.0.0.1:9117", it.link
        );
    }
    Ok(())
}
