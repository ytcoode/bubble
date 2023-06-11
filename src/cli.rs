

use clap::Parser;
use tracing::debug;

mod help;

#[derive(Parser, Debug)]
#[command(version, about, before_help = help::BEFORE, after_help = help::AFTER)]
pub struct Cli {
    #[command(flatten)]
    pub proxy: Proxy,
    // #[command(flatten)]
    // pub auth: Auth,
}

#[derive(clap::Args, Debug)]
#[group(required = true, args = ["socks5", "http", "tunnel"])]
pub struct Proxy {
    #[command(flatten)]
    pub socks5: Socks5,

    #[command(flatten)]
    pub http: Http,

    #[command(flatten)]
    pub tunnel: Tunnel,
}

#[derive(clap::Args, Debug)]
pub struct Socks5 {
    /// Start the socks5 proxy server on the <socks5-ip>:<socks5-port> address
    #[arg(id = "socks5", long)]
    pub enabled: bool,

    /// Specify the IP address for the socks5 proxy server to listen on
    #[arg(id = "socks5-ip", long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: String,

    /// Specify the port number for the socks5 proxy server to listen on
    #[arg(id = "socks5-port", long, value_name = "PORT", default_value_t = 1080)]
    pub port: u16,
}

#[derive(clap::Args, Debug)]
pub struct Http {
    /// Start the http proxy server on the <http-ip>:<http-port> address
    #[arg(id = "http", long)]
    pub enabled: bool,

    /// Specify the IP address for the http proxy server to listen on
    #[arg(id = "http-ip", long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: String,

    /// Specify the port number for the http proxy server to listen on
    #[arg(id = "http-port", long, value_name = "PORT", default_value_t = 1081)]
    pub port: u16,

    /// Specify the tunnel server address for the http proxy server to forward requests to
    #[arg(id = "http-tunnel-addr", long, value_name = "ADDR")]
    pub tunnel_addr: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct Tunnel {
    /// Start the tunnel server on the <tunnel-ip>:<tunnel-port> address
    #[arg(id = "tunnel", long)]
    pub enabled: bool,

    /// Specify the IP address for the tunnel server to listen on
    #[arg(id = "tunnel-ip", long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: String,

    /// Specify the port number for the tunnel server to listen on
    #[arg(id = "tunnel-port", long, value_name = "PORT", default_value_t = 1082)]
    pub port: u16,
}

#[derive(clap::Args, Debug)]
pub struct Auth {
    /// Whether an authentication is required to access this proxy server
    #[arg(long)]
    pub auth_required: bool,

    /// The file containing the valid users
    #[arg(long)]
    pub auth_users: Option<String>,
}

pub fn parse() -> Cli {
    let cli = Cli::parse();
    debug!("{:#?}", cli);
    cli
}
