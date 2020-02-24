use std::net::TcpListener;
use std::net::TcpStream;

async fn echo_client(ring: &rio::Rio, s: &TcpStream) {
    let buf = vec![0_u8; 512];

    loop {
        let read_bytes = match ring.read_at(s, &buf, 0).await {
            Ok(s) => s,
            Err(e) => {
                println!("io_uring read_at error: {:?}", e);
                break;
            }
        };

        if read_bytes == 0 {
            println!("client exited: {:?}", s.peer_addr());
            break;
        }

        let reply = &buf[..read_bytes];
        match ring.write_at(s, &reply, 0).await {
            Ok(_) => {}
            Err(e) => {
                println!("io_uring write_at error: {:?}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let ring = match rio::new() {
        Ok(s) => s,
        Err(e) => panic!("Failed to create io_uring: {:?}", e),
    };

    let listener = match TcpListener::bind("127.0.0.1:5555") {
        Ok(s) => s,
        Err(e) => panic!("Listener failed: {:?}", e),
    };

    loop {
        match ring.accept(&listener).await {
            Ok(s) => {
                println!("client accepted: {:?}", s.peer_addr());
                let echo_ring = ring.clone();
                tokio::spawn(async move {
                    echo_client(&echo_ring, &s).await;
                });
            }
            Err(e) => {
                println!("io_uring accept error: {:?}", e);
                continue;
            }
        };
    }
}
