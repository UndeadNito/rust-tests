use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

const PORT: &str = "3003";

pub fn start_listener() {
    let listener_result = TcpListener::bind(format!("0.0.0.0:{}", PORT));

    if listener_result.is_err() {
        println!("failed to start rust-tests on port {}", PORT);
        println!("{:?}", listener_result.err());
        return;
    }

    let listener = listener_result.unwrap();

    println!("rust-tests listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream{
            Ok(approved_stream) => {
                thread::spawn(|| {
                    handle_stream(approved_stream)
                });
            }

            Err(error) => {
                println!("connection failed: {}", error)
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    
        match stream.peer_addr(){
            Ok(address) => { println!("listening for: {:?}", address) }
            Err(_) => { println!("listening for: {{failed to get address}}") }
        };

    let mut buffer = [0u8; 8];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("{}", String::from_utf8_lossy(&buffer));
        }

        Err(error) =>{
            println!("Failed to read from {:?}:{:?}", stream.peer_addr(), error)
        }
    }


    match stream.write("Received".as_bytes()) {
        Ok(_) => {}
        Err(_) => {}
    }
}