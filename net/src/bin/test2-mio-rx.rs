//
// Echo server using Metal IO
// For testing:
//   - use nc localhost 13265
//

use mio::net::TcpListener;
use mio::net::TcpStream;
use mio::*;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::ErrorKind;

const SERVER: Token = Token(0);

struct Client {
    socket: TcpStream,
    peer_addr: SocketAddr,
    data: [u8; 512],
}

fn main() {
    let mut sockets: HashMap<Token, Client> = HashMap::new();
    let addr = "127.0.0.1:13265".parse().unwrap();
    let mut counter: usize = 1;

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
                            Ok((socket, _)) => {
                                let peer_addr = socket.peer_addr().unwrap_or_else(|_| {
                                    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
                                });
                                let token = Token(counter);
                                counter += 1;

                                println!("Connection accepted from {}", peer_addr);

                                // register for readable events
                                poll.register(&socket, token, Ready::readable(), PollOpt::edge())
                                    .unwrap();

                                let data = [0u8; 512];
                                sockets.insert(
                                    token,
                                    Client {
                                        socket,
                                        peer_addr,
                                        data,
                                    },
                                );
                            }
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                            Err(e) => panic!("Unexpected error: {}", e),
                        }
                    }
                }
                ref token if event.readiness().is_readable() => {
                    if let Some(client) = sockets.get_mut(token) {
                        loop {
                            let read = client.socket.read(&mut client.data.as_mut());
                            match read {
                                Ok(0) => {
                                    println!("Connection from {} closed", client.peer_addr);
                                    sockets.remove(token);
                                    break;
                                }
                                Ok(len) => {
                                    println!("Read {} bytes from {}", len, client.peer_addr);
                                }
                                Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                                Err(e) => panic!("Unexpected error: {}", e),
                            }
                        }
                    } else {
                        unreachable!("Could not find client with token {}", token.0);
                    }

                    // make sure client has not disconnected
                    if let Some(client) = sockets.get_mut(token) {
                        // register for writeable events
                        poll.reregister(
                            &client.socket,
                            *token,
                            Ready::writable(),
                            PollOpt::edge() | PollOpt::oneshot(),
                        )
                        .unwrap();
                    } else {
                        println!("Client with token {} has been closed", token.0);
                    }
                }
                ref token if event.readiness().is_writable() => {
                    if let Some(client) = sockets.get_mut(token) {
                        println!("Write back to {}", client.peer_addr);
                        if client.socket.write_all(&client.data).is_ok() {
                            // get ready for the next echo: clear data
                            client.data.iter_mut().for_each(|x| *x = 0);
                            poll.reregister(
                                &client.socket,
                                *token,
                                Ready::readable(),
                                PollOpt::edge(),
                            )
                            .unwrap();
                        } else {
                            println!(
                                "Writing failed to {}, closing this socket",
                                client.peer_addr
                            );
                            sockets.remove(token);
                        }
                    } else {
                        unreachable!("Could not find client with token {}", token.0);
                    }
                }
                x => unreachable!("Could not find client with token {}", x.0)
            }
        }
    }
}
