use std::env;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Rss { channel: Channel }

#[derive(Deserialize, Debug)]
struct Channel { item: Vec<Item> }

#[derive(Deserialize, Debug)]
struct Item {
    title: String,
    link: String,
    size: Option<u64>,
    // Use an internal struct to catch the torznab attributes
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
    let url = format!("http://127.0.0.1:9117/api/v2.0/indexers/all/results/torznab/api?apikey={}&q={}", api_key, query);

    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.text().await?;

    let rss: Rss = from_str(&body)?;

    for it in rss.channel.item {
        // Extract seeds and peers from the attributes vector
        let seeds = it.attributes.iter()
        .find(|a| a.name == "seeders")
        .map(|a| a.value.as_str()).unwrap_or("-1");

        let leech = it.attributes.iter()
        .find(|a| a.name == "peers")
        .map(|a| a.value.as_str()).unwrap_or("-1");

        println!("{}|{}|{}|{}|{}|{}|{}",
                 it.link,
                 it.title.replace('|', ""),
                 it.size.unwrap_or(0),
                 seeds,
                 leech,
                 "http://127.0.0.1:9117",
                 it.link
        );
    }
    Ok(())
}
