use std::net::IpAddr;
use tokio::signal;

mod cli;
mod init;
mod proxy;

#[tokio::main]
async fn main() {
    init::init();

    let cli = cli::parse();

    if cli.proxy.socks5.enabled {
        tokio::spawn(proxy::socks5::start((
            cli.proxy.socks5.ip.parse::<IpAddr>().expect("socks5-ip"),
            cli.proxy.socks5.port,
        )));
    }

    if cli.proxy.http.enabled {
        tokio::spawn(proxy::http::start(
            (
                cli.proxy.http.ip.parse::<IpAddr>().expect("http-ip"),
                cli.proxy.http.port,
            ),
            cli.proxy.http.tunnel_addr,
        ));
    }

    if cli.proxy.tunnel.enabled {
        tokio::spawn(proxy::tunnel::start((
            cli.proxy.tunnel.ip.parse::<IpAddr>().expect("tunnel-ip"),
            cli.proxy.tunnel.port,
        )));
    }

    signal::ctrl_c().await.expect("signal::ctrl_c");
}
