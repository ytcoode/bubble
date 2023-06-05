use std::net::SocketAddr;

use anyhow::{anyhow, bail, ensure, Context};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{self, TcpSocket, TcpStream},
};
use tracing::{debug, info, warn};

use super::util;

const VERSION: u8 = 0x05;

pub async fn process(mut socket: TcpStream) {
    let addrs = util::tcp_stream_addrs(&socket, false);
    // debug!("{}: new connection", addrs);

    match handle(&mut socket).await {
        Err(e) => warn!("{addrs} - error: {e:?}"),
        Ok((tx, rx)) => info!("{} - sent: {tx}, received: {rx}", addrs),
    }
}

async fn handle(socket: &mut TcpStream) -> anyhow::Result<(u64, u64)> {
    authenticate(socket).await?;
    let mut socket2 = connect(socket).await?;

    let r = io::copy_bidirectional(socket, &mut socket2)
        .await
        .context("io::copy_bidirectional")?;

    Ok(r)
}

async fn authenticate(socket: &mut TcpStream) -> anyhow::Result<()> {
    const NO_AUTHENTICATION_REQUIRED: u8 = 0x00;
    // const USERNAME_PASSWORD: u8 = 0x02;
    const NO_ACCEPTABLE_METHODS: u8 = 0xff;

    // +----+----------+----------+
    // |VER | NMETHODS | METHODS  |
    // +----+----------+----------+
    // | 1  |    1     | 1 to 255 |
    // +----+----------+----------+

    let mut buf = [0; 255];

    socket
        .read_exact(&mut buf[..2])
        .await
        .context("authenticate read ver/nmethods")?;

    ensure!(
        buf[0] == VERSION,
        "authenticate: invalid version: {}",
        buf[0]
    );

    let nmethods = buf[1] as usize;
    let methods = &mut buf[..nmethods];
    socket
        .read_exact(methods)
        .await
        .context("authenticate read methods")?;

    debug!(
        "{} - methods - {:?}",
        util::tcp_stream_addrs(socket, false),
        methods
            .iter()
            .map(|v| util::authentication_method_name(*v))
            .collect::<Vec<_>>()
    );

    // +----+--------+
    // |VER | METHOD |
    // +----+--------+
    // | 1  |   1    |
    // +----+--------+

    if !methods.contains(&NO_AUTHENTICATION_REQUIRED) {
        // TODO
        socket
            .write_all(&[VERSION, NO_ACCEPTABLE_METHODS])
            .await
            .context("authenticate write NO_ACCEPTABLE_METHODS")?;
        bail!("invalid authentication method");
    }

    socket
        .write_all(&[VERSION, NO_AUTHENTICATION_REQUIRED])
        .await
        .context("authenticate write NO_ACCEPTABLE_METHODS")?;

    // authenticate_using_username_password(socket).await?;

    Ok(())
}

#[allow(dead_code)] // TODO
async fn authenticate_using_username_password(socket: &mut TcpStream) -> anyhow::Result<()> {
    const VER: u8 = 1;

    // +----+------+----------+------+----------+
    // |VER | ULEN |  UNAME   | PLEN |  PASSWD  |
    // +----+------+----------+------+----------+
    // | 1  |  1   | 1 to 255 |  1   | 1 to 255 |
    // +----+------+----------+------+----------+

    let mut b1 = [0; 255];
    socket
        .read_exact(&mut b1[..2])
        .await
        .context("authenticate_using_username_password: read first 2 bytes")?;

    ensure!(
        b1[0] == VER,
        "authenticate_using_username_password: invalid VER {}",
        b1[0]
    );

    let ulen = b1[1] as usize;
    let uname = &mut b1[..ulen];
    socket
        .read_exact(uname)
        .await
        .context("authenticate_using_username_password: read username")?;

    let plen = socket
        .read_u8()
        .await
        .context("authenticate_using_username_password: read plen")? as usize;

    let mut passwd = vec![0; plen];
    socket
        .read_exact(&mut passwd)
        .await
        .context("authenticate_using_username_password: read password")?;

    debug!(
        "username: {}, password: {}",
        std::str::from_utf8(uname).unwrap(),
        std::str::from_utf8(passwd.as_slice()).unwrap()
    );

    // +----+--------+
    // |VER | STATUS |
    // +----+--------+
    // | 1  |   1    |
    // +----+--------+

    let status = if uname == passwd.as_slice() { 0 } else { 1 };
    socket
        .write_all(&[VER, status])
        .await
        .context("authenticate_using_username_password: write replay")?;

    Ok(())
}

async fn connect(socket: &mut TcpStream) -> anyhow::Result<TcpStream> {
    // TODO minimize the number of system calls
    const CMD_CONNECT: u8 = 0x01;

    const ATYP_IP_V4_ADDR: u8 = 0x01;
    const ATYP_IP_V6_ADDR: u8 = 0x04;
    const ATYP_DOMAINNAME: u8 = 0x03;

    // +----+-----+-------+------+----------+----------+
    // |VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |
    // +----+-----+-------+------+----------+----------+
    // | 1  |  1  | X'00' |  1   | Variable |    2     |
    // +----+-----+-------+------+----------+----------+

    let mut buf = [0; 255];

    socket
        .read_exact(&mut buf[..4])
        .await
        .context("connect: read basics")?;

    ensure!(buf[0] == VERSION, "connect: invalid VERSION: {}", buf[0]);
    ensure!(buf[1] == CMD_CONNECT, "connect: invalid CMD: {}", buf[1]);
    ensure!(buf[2] == 0x00, "connect: invalid RSV: {}", buf[2]);

    let dst_addr = match buf[3] {
        ATYP_IP_V4_ADDR => {
            let mut ip = [0; 4];
            socket
                .read_exact(&mut ip)
                .await
                .context("connect: read ipv4 addr")?;

            let port = socket.read_u16().await.context("connect: read ipv4 port")?;
            let addr = (ip, port).into();
            debug!(
                "{} - connect to: {}",
                util::tcp_stream_addrs(socket, false),
                addr
            );
            addr
        }

        ATYP_IP_V6_ADDR => {
            let mut ip = [0; 16];
            socket
                .read_exact(&mut ip)
                .await
                .context("connect: read ipv6 addr")?;

            let port = socket.read_u16().await.context("connect: read ipv6 port")?;
            let addr = (ip, port).into();
            debug!(
                "{} - connect to: {}",
                util::tcp_stream_addrs(socket, false),
                addr
            );
            addr
        }

        ATYP_DOMAINNAME => {
            let n = socket
                .read_u8()
                .await
                .context("connect: read domainname length")? as usize;

            let domain_name = &mut buf[..n];

            socket
                .read_exact(domain_name)
                .await
                .context("connect: read domainname")?;

            let domain_name =
                std::str::from_utf8(domain_name).context("connect: str::from_utf8(domain_name)")?;

            let port = socket
                .read_u16()
                .await
                .context("connect: read domain_name port")?;

            debug!(
                "{} - connect to: {}:{}",
                util::tcp_stream_addrs(socket, false),
                domain_name,
                port
            );

            let iter = net::lookup_host((domain_name, port))
                .await
                .context("connect: lookup_host")?;

            let mut addr = None;
            for a in iter {
                addr = Some(a);
                if a.is_ipv4() {
                    break;
                }
            }
            addr.ok_or_else(|| anyhow!("connect: lookup_host: empty: {}:{}", domain_name, port))?
        }

        _ => bail!("connect: invalid ATYP: {}", buf[3]),
    };

    let socket2 = match dst_addr {
        SocketAddr::V4(_) => TcpSocket::new_v4().context("connect: TcpSocket::new_v4")?,
        SocketAddr::V6(_) => TcpSocket::new_v6().context("connect: TcpSocket::new_v6")?,
    };

    {
        // TODO
        // let mut local_addr = socket.local_addr().context("connect: socket.local_addr")?;
        // if dst_addr.is_ipv4() && local_addr.is_ipv4() || dst_addr.is_ipv6() && local_addr.is_ipv6()
        // {
        //     local_addr.set_port(0);
        //     socket2.bind(local_addr).context("connect: socket2.bind")?;
        // }
    }

    let socket2 = socket2
        .connect(dst_addr)
        .await
        .context("connect: socket2.connect")?;

    // +----+-----+-------+------+----------+----------+
    // |VER | REP |  RSV  | ATYP | BND.ADDR | BND.PORT |
    // +----+-----+-------+------+----------+----------+
    // | 1  |  1  | X'00' |  1   | Variable |    2     |
    // +----+-----+-------+------+----------+----------+

    buf[0] = VERSION;
    buf[1] = 0x00;
    buf[2] = 0x00;

    let local_addr = socket2
        .local_addr()
        .context("connect: socket2.local_addr")?;

    let reply = match local_addr {
        SocketAddr::V4(a) => {
            let ip = a.ip().octets();
            let port = a.port().to_ne_bytes();

            buf[3] = ATYP_IP_V4_ADDR;
            buf[4..8].copy_from_slice(&ip);
            buf[8..10].copy_from_slice(&port);
            &buf[..10]
        }

        SocketAddr::V6(a) => {
            let ip = a.ip().octets();
            let port = a.port().to_ne_bytes();

            buf[3] = ATYP_IP_V6_ADDR;
            buf[4..20].copy_from_slice(&ip);
            buf[20..22].copy_from_slice(&port);
            &buf[..22]
        }
    };

    socket
        .write_all(reply)
        .await
        .context("connect: write replay")?;

    debug!(
        "{} - {}",
        util::tcp_stream_addrs(socket, false),
        util::tcp_stream_addrs(&socket2, true)
    );

    Ok(socket2)
}
