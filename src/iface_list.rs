#[derive(Debug)]
pub(crate) struct Iface {
    pub(crate) name: String,
    pub(crate) ip: String,
}

impl std::fmt::Display for Iface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ip)
    }
}

impl Iface {
    pub(crate) fn get_all() -> impl Iterator<Item = Self> {
        local_ip_address::list_afinet_netifas()
            .unwrap_or_default()
            .into_iter()
            .filter(|(name, ip)| ip.is_ipv4() && name != "lo")
            .map(|(name, ip)| Self {
                name,
                ip: format!("{}", ip),
            })
    }
}
