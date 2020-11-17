use std::env;
use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpListener,TcpStream};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return
    }
    let listener = TcpListener::bind(&args[1]).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    match handle_auth(stream) {
                        Err(v) => println!("[auth] error {}", v),
                        _ => {},
                    }
                });    
            }
            Err(e) => {
                println!("Error incoming stream {}", e);
            }
        }
       
    }
}

fn handle_auth(mut stream: TcpStream) -> Result<()> {
    let mut u_id = [0; 1];
    stream.read(&mut u_id)?;
    let id = u_id[0];

    println!("New authorization");

    if id == 1 {
        stream.write( String::from("127.0.0.1:25450").as_bytes())?;
    } else {
        stream.write(&[0])?;
    }
    
    Ok(())
}
