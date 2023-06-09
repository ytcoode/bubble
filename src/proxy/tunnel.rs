use std::{net::SocketAddr, time::Duration};

use anyhow::Context;

use tokio::io::{self, AsyncReadExt};
use tokio::{
    net::{TcpListener, TcpStream},
    time,
};
use tracing::{error, instrument};

pub async fn start<A>(addr: A)
where
    A: Into<SocketAddr>,
{
    let l = TcpListener::bind(addr.into())
        .await
        .expect("TcpListener::bind");

    loop {
        match l.accept().await {
            Err(e) => {
                error!("An error occurred while calling listener.accept: {}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
            Ok((s, _)) => {
                tokio::spawn(handle_socket(s));
            }
        }
    }
}

#[instrument(skip_all, fields(
    peer_addr = %s.peer_addr().unwrap(),
    local_addr = %s.local_addr().unwrap()
))]
async fn handle_socket(mut s: TcpStream) -> anyhow::Result<()> {
    let len = s.read_u16().await.context("s.read_u16")? as usize;
    let mut addr = vec![0; len];

    s.read_exact(&mut addr).await.context("s.read_exact")?;
    let addr = std::str::from_utf8(&addr).context("from_utf8")?;

    let mut server = TcpStream::connect(addr).await.context("connect")?;
    io::copy_bidirectional(&mut s, &mut server)
        .await
        .context("io::copy_bidirectional")?;

    Ok(())
}
