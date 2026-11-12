use std::net::{IpAddr, SocketAddr};

/// Transform an IP/PORT string to a SocketAddr structure.
pub fn parse_ip_port(s: &str) -> Result<SocketAddr, String> {
    if let Ok(addr) = s.parse::<SocketAddr>() {
        return Ok(addr);
    }
    let (host, port) = s
        .rsplit_once('/')
        .ok_or_else(|| "expected IP/PORT".to_string())?;
    let host = host.trim_start_matches('[').trim_end_matches(']');
    let ip: IpAddr = host
        .parse()
        .map_err(|e| format!("invalid IP address {host:?}: {e}"))?;
    let port: u16 = port
        .parse()
        .map_err(|e| format!("invalid port {port:?}: {e}"))?;
    Ok(SocketAddr::new(ip, port))
}