extern crate websrv;
use websrv::ThreadPool;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let port = get_user_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    println!("Listen on {:?}", listener);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_req(stream);
        });
    }
}

fn handle_req(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status, template) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "data/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "data/404.html")
    };

    let rsp = prepare_rsp(status, template);
    stream.write(rsp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn prepare_rsp(status: &str, template: &str) -> String {
    let mut file = File::open(template).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    format!("{}{}", status, contents)
}

fn get_user_port() -> u16 {
    let mut port = String::new();

    println!("Specify TCP port:");
    io::stdin().read_line(&mut port).unwrap();

    port.trim().parse().unwrap()
}
