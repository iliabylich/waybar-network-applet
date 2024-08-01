pub(crate) struct IfaceList(Vec<Iface>);

pub(crate) struct Iface {
    pub(crate) name: String,
    pub(crate) ip: String,
}

static mut IFACES: IfaceList = IfaceList(vec![]);

impl std::fmt::Display for Iface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ip)
    }
}

impl IfaceList {
    pub(crate) fn init() {
        let ifaces = local_ip_address::list_afinet_netifas()
            .unwrap_or_default()
            .into_iter()
            .filter(|(name, ip)| ip.is_ipv4() && name != "lo")
            .map(|(name, ip)| Iface {
                name,
                ip: format!("{}", ip),
            })
            .collect::<Vec<_>>();

        unsafe { IFACES = IfaceList(ifaces) };
    }

    pub(crate) fn get() -> &'static [Iface] {
        unsafe { IFACES.0.as_slice() }
    }

    pub(crate) fn at(idx: usize) -> &'static Iface {
        unsafe { &IFACES.0[idx] }
    }
}
