use clap::Parser;
use log::debug;

mod util;

#[derive(Parser, Debug)]
#[command(version, about, before_help = util::before_help(), after_help = util::after_help())]
pub struct Cli {
    #[command(flatten)]
    pub proxy: Proxy,
    // #[command(flatten)]
    // pub auth: Auth,
}

#[derive(clap::Args, Debug)]
#[group(required = true, args(["socks5", "http", "https"]))]
pub struct Proxy {
    #[command(flatten)]
    pub socks5: Socks5,

    #[command(flatten)]
    pub http: Http,

    #[command(flatten)]
    pub https: Https,
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
    #[arg(id = "http-id", long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: String,

    /// Specify the port number for the http proxy server to listen on
    #[arg(id = "http-port", long, value_name = "PORT", default_value_t = 1081)]
    pub port: u16,
}

#[derive(clap::Args, Debug)]
pub struct Https {
    /// Start the https proxy server on the <https-ip>:<https-port> address
    #[arg(id = "https", long)]
    pub enabled: bool,

    /// Specify the IP address for the https proxy server to listen on
    #[arg(id = "https-id", long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: String,

    /// Specify the port number for the https proxy server to listen on
    #[arg(id = "https-port", long, value_name = "PORT", default_value_t = 1082)]
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
    let r = Cli::parse();
    debug!("{:#?}", r);
    r
}