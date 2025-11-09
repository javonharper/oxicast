use std::net::IpAddr;

pub fn get_host_ip() -> Option<IpAddr> {
    local_ip_address::list_afinet_netifas()
        .unwrap_or_default()
        .into_iter()
        .map(|(_, ip)| ip)
        .filter(|ip| !ip.is_loopback() && ip.is_ipv4())
        .next()
}
