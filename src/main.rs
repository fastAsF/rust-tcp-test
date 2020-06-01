use std::thread;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::net::{TcpListener, TcpStream, Shutdown};
use simple_user_input::get_input;

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; filename.len() + metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}


fn handle_messages(mut stream: TcpStream) {
    let mut data: Vec<u8> = vec![0; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            println!("What the server got is: {:#?}", &size);
            stream.write(&data[0..size]).unwrap();
            stream.shutdown(Shutdown::Both).unwrap();
            true
        },
        Err(error) => {
            println!("Something Error: {:?}", error);
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let commands: Vec<String> = env::args().collect();
    if &commands[1] == "server" {
        let listener = TcpListener::bind("0.0.0.0:3334").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move|| {
                        handle_messages(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    } else {
        match TcpStream::connect("localhost:3334") {
            Ok(mut stream) => {
                let input: String = get_input("Please type something...");
                println!("The input is: {:?}", input);
    
                stream.write(input.as_bytes()).unwrap();
                let mut data: Vec<u8> = vec![0; 1024];
                match stream.read(&mut data) {
                    Ok(_) => {
                        println!("The response is {:?}", &data)
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
        println!("Terminated.");
    }
}
