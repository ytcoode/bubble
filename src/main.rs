use std::net::IpAddr;
use tokio::signal;

mod cli;
mod proxy;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = cli::parse();

    if cli.proxy.socks5.enabled {
        tokio::spawn(proxy::socks5::start((
            cli.proxy.socks5.ip.parse::<IpAddr>().expect("socks5-ip"),
            cli.proxy.socks5.port,
        )));
    }

    if cli.proxy.http.enabled {
        tokio::spawn(proxy::http::start((
            cli.proxy.http.ip.parse::<IpAddr>().expect("http-ip"),
            cli.proxy.http.port,
        )));
    }

    if cli.proxy.https.enabled {
        tokio::spawn(proxy::https::start((
            cli.proxy.https.ip.parse::<IpAddr>().expect("https-ip"),
            cli.proxy.https.port,
        )));
    }

    signal::ctrl_c().await.expect("signal::ctrl_c");
}
