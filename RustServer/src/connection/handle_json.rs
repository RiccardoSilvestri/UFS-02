use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{Error as JsonError, Value};

// prettyfies the json, writes it to file and returns it as a string
pub fn handle_json(json_string : String, filename :&str) -> String{
    eprintln!("{}", json_string);
    let json_result: Result<Value, JsonError> = serde_json::from_str(&json_string);
    return match json_result {
        Ok(json) => {
            // Open the file in read mode
            let mut file = OpenOptions::new().read(true).open(filename).unwrap_or_else(|_| File::create(filename).expect("Failed to create file"));
            // Read the existing JSON data
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read file");
            // Parse the existing JSON data into an array
            let mut array: Vec<Value> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
            // Append the new JSON data to the array
            array.push(json);
            // Open the file in write mode
            let mut file = OpenOptions::new().write(true).truncate(true).open(filename).expect("Failed to open file");
            // Write the array back to the file in a pretty-printed format
            writeln!(file, "{}", serde_json::to_string_pretty(&array).expect("Failed to serialize JSON")).expect("Failed to write to file");
            println!("Wrote to json");
            let response = serde_json::to_string_pretty(&array.last().unwrap()).expect("Failed to serialize JSON");
            response
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            eprintln!("{}", json_string);
            "Error".to_string()
            // TODO: send a response back to client
        }
    }
}