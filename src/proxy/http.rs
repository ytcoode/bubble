use std::{net::SocketAddr, time::Duration};

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{body, client, server};
use hyper::{Method, Request, Response};
use tokio::io::AsyncWriteExt;
use tokio::{
    net::{TcpListener, TcpStream},
    time,
};
use tracing::{debug, error};

pub async fn start<A>(addr: A, tunnel_addr: Option<String>)
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
                tokio::spawn(handle_socket(s, tunnel_addr.clone()));
            }
        }
    }
}

async fn handle_socket(s: TcpStream, tunnel_addr: Option<String>) {
    if let Err(err) = server::conn::http1::Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .serve_connection(
            s,
            service_fn(|req: Request<body::Incoming>| async {
                proxy(req, tunnel_addr.clone()).await
            }),
        )
        .with_upgrades()
        .await
    {
        error!("Failed to serve connection: {:?}", err);
    }
}

async fn proxy(
    req: Request<body::Incoming>,
    tunnel_addr: Option<String>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    debug!("req: {:?}", req);

    if req.method() == Method::CONNECT {
        match req.uri().authority().map(|auth| auth.to_string()) {
            None => {
                error!("missing authority: {}", req.uri());
                let mut resp = Response::new(full("missing authority"));
                *resp.status_mut() = http::StatusCode::BAD_REQUEST;
                Ok(resp)
            }

            Some(addr) => {
                tokio::task::spawn(async move {
                    match hyper::upgrade::on(req).await {
                        Ok(upgraded) => {
                            if let Err(e) = tunnel(upgraded, addr, tunnel_addr).await {
                                error!("tunnel error: {}", e);
                            };
                        }
                        Err(e) => error!("upgrade error: {}", e),
                    }
                });
                Ok(Response::new(empty()))
            }
        }
    } else {
        let host = req.uri().host().expect("uri has no host");
        let port = req.uri().port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await.unwrap();

        let (mut sender, conn) = client::conn::http1::Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .handshake(stream)
            .await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                error!("Connection failed: {:?}", err);
            }
        });

        let resp = sender.send_request(req).await?;
        Ok(resp.map(|b| b.boxed()))
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn tunnel(
    mut upgraded: Upgraded,
    addr: String,
    tunnel_addr: Option<String>,
) -> std::io::Result<()> {
    let mut server = match tunnel_addr {
        Some(a) => {
            let mut s = TcpStream::connect(a).await?;
            s.write_u16(addr.len() as u16).await?;
            s.write_all(addr.as_bytes()).await?;
            s
        }
        None => TcpStream::connect(addr).await?,
    };

    tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    Ok(())
}
