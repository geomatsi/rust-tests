use std::io;
use std::net::SocketAddr;
use std::net::TcpListener;

fn main() {
    let port = get_user_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();

    println!("Listen on {:?}", listener);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection!");
    }
}

fn get_user_port() -> u16 {
    let mut port = String::new();

    println!("Specify TCP port:");
    io::stdin().read_line(&mut port).unwrap();

    port.trim().parse().unwrap()
}
