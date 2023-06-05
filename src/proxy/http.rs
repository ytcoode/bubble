use std::net::SocketAddr;

use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

pub async fn start<A>(addr: A)
where
    A: Into<SocketAddr>,
{
    let l = TcpListener::bind(addr.into()).await.unwrap();

    loop {
        let (s, _) = l.accept().await.unwrap();
        tokio::spawn(handle_socket(s));
    }
}

async fn handle_socket(mut s: TcpStream) {
    match io::copy(&mut s, &mut io::stdout()).await {
        Err(e) => println!("io::copy::Err: {}", e),
        Ok(n) => println!("io::copy.N: {}", n),
    }
}
