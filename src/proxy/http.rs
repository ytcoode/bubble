use std::{net::SocketAddr, time::Duration};

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{body, client, server};
use hyper::{Method, Request, Response};
use tokio::{
    net::{TcpListener, TcpStream},
    time,
};
use tracing::{debug, error};

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

async fn handle_socket(s: TcpStream) {
    if let Err(err) = server::conn::http1::Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .serve_connection(s, service_fn(proxy))
        .with_upgrades()
        .await
    {
        error!("Failed to serve connection: {:?}", err);
    }
}

async fn proxy(
    req: Request<body::Incoming>,
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
                            if let Err(e) = tunnel(upgraded, addr).await {
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

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;

    tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    Ok(())
}
