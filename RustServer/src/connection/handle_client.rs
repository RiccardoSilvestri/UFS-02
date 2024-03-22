use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::connection::handle_json::write_json;
use super::send_utf::*;
use super::read_stream::*;
use crate::connection::user::register::*;
use crate::connection::user::login::*;
use crate::connection::notes::filter_by_author::*;
use super::notes::remove_note::remove_note;

pub(crate) fn handle_client(mut stream: TcpStream, file_access: Arc<Mutex<()>>) {
    let mut logged = false;
    let mut return_msg = "Invalid request";
    loop {
        while !logged{
            let request = read_utf(&mut stream);
            if request.is_empty(){ return };
            println!("Received: {}", request);
            send_utf("request received".to_string(), stream.try_clone().unwrap());
            if request.eq_ignore_ascii_case("1") {
                let registration = read_utf(&mut stream);
                println!("registration request: {}", registration);
                let result = register(registration, file_access.clone());
                match result{
                    Ok(_) => {
                        return_msg = "Registration succeeded";
                        logged = true;
                    },
                    Err(e) => println!("Registration failed: {}", e),
                }
                println!("Logged: {}", logged)
            } else if request.eq_ignore_ascii_case("2") {
                let credentials = read_utf(&mut stream);
                println!("login request: {}", credentials);
                let result = login(credentials);
                match result{
                    Ok(_) => {
                        return_msg = "Login succeeded";
                        logged = true;
                    },
                    Err(e) => println!("Login failed: {}", e),
                }
            } else { println!("Invalid request") }
            println!("{}", return_msg);
            println!("Logged: {}", logged);
            let byte_value = logged as u8;
            stream.write(&[byte_value]).expect("can't send boolean to stream");
        }
        let request = read_utf(&mut stream);
        if request.is_empty(){ return };
        println!("Received: {}", request);

        // send all user's notes to the client
        let response = filter_by_author("received.json", "Paolo", file_access.clone()).unwrap().to_string();
        println!("{}", response);
        send_utf(response, stream.try_clone().unwrap());
        match request.as_str(){
            "1" => {
                let request = read_utf(&mut stream);
                if request.is_empty(){ return };
                println!("create a note");
                // create a note
                write_json(request, "received.json", file_access.clone());
            },
            "2" => {
                // the client should send a json containing only author and title
                let request = read_utf(&mut stream);
                if request.is_empty(){ return };
                remove_note("received.json", &*request, file_access.clone()).unwrap()
            },
            _ => println!("invalid request")
        }

    }
}
