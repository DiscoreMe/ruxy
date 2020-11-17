use std::env;
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io::Result;
use std::io::prelude::{Write,Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return
    }

    let port = &args[1];
    let timeout = Duration::from_secs(10);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555);
    
    match auth(&socket, timeout) {
        Err(v) => println!("Error auth {}", v),
        _ => println!("Success")
    }
}

fn auth(socket: &SocketAddr, timeout: Duration) -> Result<()> {
    let mut stream: TcpStream = TcpStream::connect_timeout(&socket, timeout)?;

    let buf = vec![1];
    stream.write(&buf)?;
    
    let mut url = String::new();
    stream.read_to_string(&mut url)?;

    println!("Get url: {}", url);

    Ok(())
}
