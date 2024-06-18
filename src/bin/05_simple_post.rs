use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create a client instance
    let client = reqwest::Client::new();

    // Send a POST request
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    // Print the status code and response body
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body: \n{}", body);

    Ok(())
}
