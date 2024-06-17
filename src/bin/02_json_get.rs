//! A simple Crate using GET to read data from REST API
//!
use reqwest;
use serde_json;
use std::time::Instant;

fn print_heading(heading: &str) {
    //! Prints the header and underlines it based on the length of the Header.
    let text = heading;
    let underline = "_".repeat(text.len());
    println!("\n{}\n{}\n", text, underline);
}

fn read_api_as_json(api_url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    //! Similar to previous method, instead of returning TEXT it returns JSON value.

    // let response_json = reqwest::blocking::get(api_url)
    //     .unwrap()
    //     .json::<serde_json::Value>()
    //     .unwrap();

    let client = reqwest::blocking::Client::new();
    let response_json = client.get(api_url).send()?.json::<serde_json::Value>()?;
    Ok(response_json)
}

fn main() {
    //! Returning as JSON is better compared to TEXT.

    // let api_url = "https://api.zippopotam.us/us/08028";

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

        match read_api_as_json(api_url) {
            Ok(response_json) => {
                print_heading("JSON");
                println!("{}\n", response_json);

                // Convert JSON to String

                let response_string = serde_json::to_string(&response_json).unwrap();
                print_heading("JSON TO STRING");
                println!("{}\n", response_string);

                print_heading("Individual Values");

                //Individual Values
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
                // Handle the error, log it, or perform a fallback action
            }
        }
    }

    //Capture elapsed time
    let duration = start.elapsed();

    println!("\n\n Synchronous version took: {:?}", duration);
}
