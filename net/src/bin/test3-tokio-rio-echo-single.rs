use std::net::TcpListener;
use std::net::TcpStream;

async fn echo_client(ring: &rio::Rio, a: &TcpStream, b: &TcpStream) {
    let buf = vec![0_u8; 512];
    loop {
        let read_bytes = match ring.read_at(a, &buf, 0).await {
            Ok(s) => s,
            Err(e) => {
                println!("io_uring read_at error: {:?}", e);
                break;
            }
        };

        if read_bytes == 0 {
            println!("client exited: {:?}", a.peer_addr());
            break;
        }

        println!("recv: {}", String::from_utf8_lossy(&buf[..read_bytes - 1]));
        let buf = &buf[..read_bytes];
        match ring.write_at(b, &buf, 0).await {
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
        let stream = match ring.accept(&listener).await {
            Ok(s) => {
                println!("client accepted: {:?}", s.peer_addr());
                s
            }
            Err(e) => {
                println!("io_uring accept error: {:?}", e);
                continue;
            }
        };

        echo_client(&ring, &stream, &stream).await;
    }
}
