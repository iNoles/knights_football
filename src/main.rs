use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SpaceX {
    name: String,
    date_unix: u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.spacexdata.com/v5/launches")
        .await?
        .json::<Vec<SpaceX>>()
        .await?;
    for launch in response {
        println!("{}", launch.name);
        println!("{}", launch.date_unix);
        println!("---------")
    }
    Ok(())
}
