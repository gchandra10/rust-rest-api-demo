//! A simple Crate using GET to read data from REST API
//!
use reqwest;

fn print_heading(heading: &str) {
    //! Prints the header and underlines it based on the length of the Header.
    let text = heading;
    let underline = "_".repeat(text.len());
    println!("\n{}\n{}\n", text, underline);
}

fn read_api_as_text(api_url: &str) -> String {
    //! Returns the JSON as a String.
    //! This unwrap() is not suitable for Production.
    //! .unwrap() The unwrap() method is called on the Result returned by reqwest::blocking::get. If the request is successful (i.e., it returns Ok), unwrap() extracts the Response value. If the request fails (i.e., it returns Err), unwrap() will cause the program to panic and terminate. This is useful for quick error handling in simple scripts but is generally not recommended for production code because it doesn't handle errors gracefully.
    //! .text() method is called on the Response object. This method reads the entire response body and returns a Result<String, reqwest::Error>, where String is the response body as text.
    //! .unwrap() method is called again on the Result returned by text(). If the conversion is successful (i.e., it returns Ok), unwrap() extracts the String value. If the conversion fails (i.e., it returns Err), unwrap() will cause the program to panic and terminate.
    let response = reqwest::blocking::get(api_url).unwrap().text().unwrap();
    response
}

fn read_api_as_text_prod(api_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    //! Returns the JSON as a String.
    //! client.get(api_url): Prepares the GET request for the specified URL.
    //! .send()?: Sends the request and waits for the response. The ? operator handles any errors that might occur.
    //! .text()?: Converts the response body to a String. Again, the ? operator handles any errors.

    // let response  = reqwest::blocking::get(api_url)?.text()?;

    let client = reqwest::blocking::Client::new();
    let response = client.get(api_url).send()?.text()?;
    Ok(response)
}

fn main() {
    //! Returning as JSON is better compared to TEXT.

    let api_url = "https://api.zippopotam.us/us/08028";

    let response = read_api_as_text(api_url);
    print_heading("TEXT");
    println!("{}\n", response);

    let response_result_string = read_api_as_text_prod(api_url).unwrap();
    print_heading("TEXT (PROD)");
    println!("{}\n", response_result_string);
}
