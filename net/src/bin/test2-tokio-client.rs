use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let mut pend = vec![];

    for i in 0..100 {
        pend.push(tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:5555").await {
                let msg = format!("stream: {}", i);

                if let Ok(_) = stream.write_all(msg.as_bytes()).await {
                    println!("spawned stream: {}", i);
                }
            }
        }));
    }

    while let Some(t) = pend.pop() {
        println!("joined stream: success={:?}", t.await);
    }
}
