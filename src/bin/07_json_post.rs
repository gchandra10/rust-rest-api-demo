use reqwest::Error;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a HashMap to store the JSON payload
    let mut map = HashMap::new();
    map.insert("name", "item via rust");
    map.insert("body", "json");

    // Create a client instance
    let client = reqwest::Client::new();

    // Send a POST request with JSON body
    let res = client.post("http://127.0.0.1:5002/items")
        .json(&map)
        .send()
        .await?;

    // Print the status code and response body
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: \n{}", body);

    Ok(())
}
