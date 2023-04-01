use log::error;
use tokio::net::TcpStream;

pub fn tcp_stream_addrs(s: &TcpStream, client: bool) -> String {
    let local_addr = match s.local_addr() {
        Ok(a) => a.to_string(),
        Err(e) => {
            error!("TcpStream.local_addr: {e} - client: {client}");
            "ERROR".to_string()
        }
    };

    let peer_addr = match s.peer_addr() {
        Ok(a) => a.to_string(),
        Err(e) => {
            error!("TcpStream.peer_addr: {e} - client: {client}");
            "ERROR".to_string()
        }
    };

    if client {
        format!("[{} => {}]", local_addr, peer_addr)
    } else {
        format!("[{} => {}]", peer_addr, local_addr)
    }
}

pub fn authentication_method_name(id: u8) -> String {
    match id {
        0x00 => "NO AUTHENTICATION REQUIRED".to_string(),
        0x01 => "GSSAPI".to_string(),
        0x02 => "USERNAME/PASSWORD".to_string(),
        0x03..=0x7f => format!("{id} (IANA ASSIGNED)"),
        0x80..=0xfe => format!("{id} (RESERVED FOR PRIVATE METHODS)"),
        0xff => format!("{id} (NO ACCEPTABLE METHODS)"),
    }
}
