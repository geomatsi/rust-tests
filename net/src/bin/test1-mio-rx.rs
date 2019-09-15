//
// Simple Discard Protocol server using Metal IO
// For testing:
//   - use test1-mio-tx
//   - use nc localhost 13265
//

use mio::net::TcpListener;
use mio::*;
use tokio::io::ErrorKind;

const SERVER: Token = Token(0);

fn main() {
    let addr = "127.0.0.1:13265".parse().unwrap();

    // setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // create a poll instance
    let poll = Poll::new().unwrap();

    // start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();

    // create storage for events
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    loop {
                        match server.accept() {
                            Ok((_, address)) => {
                                // Discard protocol: accept and close the connection immediately
                                println!("Connection accepted from {}", address);
                            }
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                            Err(e) => panic!("Unexpected error: {}", e),
                        }
                    }
                }
                x => unreachable!("Unexpected token {:?}", x),
            }
        }
    }
}
