use mio::net::{TcpListener, TcpStream};
use mio::*;
use std::net::SocketAddr;
use std::thread;

// Setup some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

// number of test connections
const CONNS: i32 = 20;

fn main() {
    let addr = "127.0.0.1:13265".parse().unwrap();
    let mut count = 1;

    // spawn client thread
    let handle = thread::spawn(move || {
        client_thread(addr);
    });

    // setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // create a poll instance
    let poll = Poll::new().unwrap();

    // start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();

    // create storage for events
    let mut events = Events::with_capacity(1024);

    while count < CONNS {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let (_, addr) = server.accept().unwrap();
                    println!("Connection accepted {} from {}", count, addr);
                    count += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    handle.join().unwrap();
}

fn client_thread(addr: SocketAddr) {
    let mut count = 1;

    // create a poll instance
    let poll = Poll::new().unwrap();

    // create storage for events
    let mut events = Events::with_capacity(1024);

    while count < CONNS {
        // setup the client socket
        let sock = TcpStream::connect(&addr).unwrap();

        // register the socket
        poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge())
            .unwrap();

        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                CLIENT => {
                    println!("Connection {} made to {}", count, addr);
                    count += 1;
                }
                _ => unreachable!(),
            }
        }
    }
}
