use reqwest;

fn read_api_as_text_prod(api_url: &str) -> Result<(String,reqwest::StatusCode), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(api_url).send()?;
    // Get the status code
    let status = response.status();

    // Get the response text
    let text = response.text()?;

    Ok((text,status))
}

fn main() {
    let api_url = "http://127.0.0.1:5002/items";

    let (response_result_string,status_code) = read_api_as_text_prod(api_url).unwrap();
    println!("{}-{:?}\n", response_result_string,status_code);
}
