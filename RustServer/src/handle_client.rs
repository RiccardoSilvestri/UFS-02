use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;

use serde_json::{Error as JsonError, Value};

pub(crate) fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; //
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from socket");
        if bytes_read == 0 { return; } // connection closed

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", request);

        // Convert received string to json
        let json_result: Result<Value, JsonError> = serde_json::from_str(&request);
        match json_result {
            Ok(json) => {
                // Open the file in read mode
                let mut file = OpenOptions::new().read(true).open("received.json").unwrap_or_else(|_| File::create("received.json").expect("Failed to create file"));

                // Read the existing JSON data
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read file");

                // Parse the existing JSON data into an array
                let mut array: Vec<Value> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());

                // Append the new JSON data to the array
                array.push(json);

                // Open the file in write mode
                let mut file = OpenOptions::new().write(true).truncate(true).open("received.json").expect("Failed to open file");

                // Write the array back to the file in a pretty-printed format
                writeln!(file, "{}", serde_json::to_string_pretty(&array).expect("Failed to serialize JSON")).expect("Failed to write to file");
                println!("Wrote to json");

                let response = serde_json::to_string_pretty(&array.last().unwrap()).expect("Failed to serialize JSON");

                let mut writer = BufWriter::new(&stream);
                let length = response.len() as u16;
                // The java client needs to read the length of the string first, then the string. (readUTF)
                writer.write(&length.to_be_bytes()).expect("Failed to write length to socket");
                writer.write(response.as_bytes()).expect("Failed to write to socket");
                writer.flush().expect("Failed to flush to socket");
            }
            Err(err) => {
                eprintln!("Failed to parse JSON: {}", err);
                // TODO: send a response back to client
            }
        }
    }
}