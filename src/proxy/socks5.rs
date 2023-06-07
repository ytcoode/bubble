use std::{net::SocketAddr, time::Duration};

use tokio::{net::TcpSocket, time};
use tracing::error;

mod connection;
mod util;

// https://www.rfc-editor.org/rfc/rfc1928
// https://www.rfc-editor.org/rfc/rfc1929

pub async fn start<A>(addr: A)
where
    A: Into<SocketAddr>,
{
    let socket = TcpSocket::new_v4().expect("TcpSocket::new_v4");

    #[cfg(unix)]
    {
        socket.set_reuseaddr(true).expect("socket.set_reuseaddr");
        socket.set_reuseport(true).expect("socket.set_reuseport");
    }

    socket.bind(addr.into()).expect("socket.bind");
    let listener = socket.listen(1024).expect("socket.listen");

    loop {
        match listener.accept().await {
            Err(e) => {
                error!("listener.accept: {:?}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
            Ok((socket, _)) => {
                tokio::spawn(connection::process(socket));
            }
        }
    }
}
