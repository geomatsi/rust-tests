//
// Simple client using mio
//

use mio::net::TcpStream;
use mio::*;

const CLIENT: Token = Token(0);
const CONNS: i32 = 20;

fn main() {
    let addr = "127.0.0.1:13265".parse().unwrap();
    let mut count = 1;

    // create a poll instance
    let poll = Poll::new().unwrap();

    // create storage for events
    let mut events = Events::with_capacity(1024);

    while count <= CONNS {
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
