use futures::stream::StreamExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5555";
    let mut listener = match TcpListener::bind(addr).await {
        Ok(s) => s,
        Err(e) => panic!("Listener failed: {:?}", e),
    };

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut conn) => {
                    println!("Accepted connection from {:?}", conn.peer_addr());

                    tokio::spawn(async move {
                        let peer = conn.peer_addr();
                        let (read_conn, _) = conn.split();
                        let mut reader = BufReader::new(read_conn);
                        let mut buffer = String::new();

                        while let Ok(n) = reader.read_line(&mut buffer).await {
                            if n == 0 {
                                break;
                            }
                        }

                        println!("Closed connection from {:?}", peer);
                    });
                }
                Err(err) => {
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:5555");

    server.await;
}
