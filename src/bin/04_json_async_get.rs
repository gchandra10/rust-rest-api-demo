use reqwest;
use serde_json;
use std::time::Instant;
use tokio;

fn print_heading(heading: &str) {
    let text = heading;
    let underline = "_".repeat(text.len());
    println!("\n{}\n{}\n", text, underline);
}

async fn read_api_as_json(api_url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response_json = client
        .get(api_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(response_json)
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://api.zippopotam.us/us/08028",
        "https://api.zippopotam.us/us/90210",
        "https://api.zippopotam.us/us/30301",
        "https://api.zippopotam.us/us/10001",
        "https://api.zippopotam.us/us/20005",
        "https://api.zippopotam.us/us/65055",
        "https://api.zippopotam.us/us/94105",
        "https://api.zippopotam.us/us/60601",
        "https://api.zippopotam.us/us/73301",
        "https://api.zippopotam.us/us/48201",
        "https://api.zippopotam.us/us/80201",
        "https://api.zippopotam.us/us/90001",
        "https://api.zippopotam.us/us/85001",
        "https://api.zippopotam.us/us/02201",
        "https://api.zippopotam.us/us/20001",
        "https://api.zippopotam.us/us/33109",
        "https://api.zippopotam.us/us/10007",
        "https://api.zippopotam.us/us/94102",
        "https://api.zippopotam.us/us/60614",
        "https://api.zippopotam.us/us/73344",
        "https://api.zippopotam.us/us/48226",
    ];

    // Start the timer
    let start = Instant::now();

    for api_url in &urls {
        print_heading(api_url);

        match read_api_as_json(api_url).await {
            Ok(response_json) => {
                print_heading("JSON");
                println!("{}\n", response_json);

                let response_string = serde_json::to_string(&response_json).unwrap();
                print_heading("JSON TO STRING");
                println!("{}\n", response_string);

                print_heading("Individual Values");

                println!("Country: {}", response_json["country"]);
                println!("Country Abbr: {}", response_json["country abbreviation"]);
                println!("Post Code: {}", response_json["post code"]);

                println!("Places: {}", response_json["places"]);
                println!("Places Array: {}", response_json["places"][0]);

                println!("Place Name: {}", response_json["places"][0]["place name"]);
                println!("Latitude: {}", response_json["places"][0]["latitude"]);
                println!("Longitude: {}", response_json["places"][0]["longitude"]);
                println!("State: {}", response_json["places"][0]["state"]);
                println!(
                    "State Abbr: {}",
                    response_json["places"][0]["state abbreviation"]
                );
            }
            Err(e) => {
                eprintln!("Error fetching JSON: {}", e);
            }
        }
    }
    //Capture elapsed time
    let duration = start.elapsed();
    println!("\n\n ASynchronous version took: {:?}", duration);
}
