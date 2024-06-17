use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

fn read_api_as_json(api_url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    //! Similar to previous method, instead of returning TEXT it returns JSON value.
    //! Returns a JSON

    let client = reqwest::blocking::Client::new();
    let response_json = client.get(api_url).send()?.json::<serde_json::Value>()?;
    Ok(response_json)
}

// Takes a JSON Value and returns a vector of HashMaps.
// Checking if the JSON value is an array.
// If it is, mapping each element to flatten_object and collecting
// the results into a vector.

fn flatten_json(json: &Value) -> Vec<HashMap<String, String>> {
    match json {
        Value::Array(arr) => arr.iter().map(flatten_object).collect(),
        _ => vec![],
    }
}

// Defining a function flatten_object that takes a JSON Value and
// returns a flattened HashMap.

fn flatten_object(obj: &Value) -> HashMap<String, String> {
    // Creating a mutable HashMap to store the flattened key-value pairs.
    let mut flat_map = HashMap::new();

    if let Value::Object(map) = obj {
        for (key, value) in map {
            match value {
                Value::String(s) => {
                    flat_map.insert(key.clone(), s.clone());
                }

                Value::Number(num) => {
                    flat_map.insert(key.clone(), num.to_string());
                }

                Value::Bool(b) => {
                    flat_map.insert(key.clone(), b.to_string());
                }

                Value::Array(arr) => {
                    flat_map.insert(key.clone(), serde_json::to_string(arr).unwrap());
                }

                Value::Null => {
                    flat_map.insert(key.clone(), "null".to_string());
                }

                _ => {} // Handle other cases if needed
            }
        }
    }
    flat_map
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_url = "http://universities.hipolabs.com/search?name=Kharkiv%20National%20University";
    let universities = read_api_as_json(api_url)?;
    let flattened_data = flatten_json(&universities);

    println!("{}\n\n", universities);
    // println!("{:?}", flattened_data);

    for university in flattened_data {
        for (key, value) in &university {
            println!("{}: {}", key, value);
        }
        println!();
    }

    Ok(())
}
