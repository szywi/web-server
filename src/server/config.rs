use std::net::{IpAddr, Ipv4Addr};

pub struct ServerConfig {
    pub address: IpAddr,
    pub port: u16,
    pub workers: usize,
    host: Option<String>,
}

impl ServerConfig {
    pub fn host(&mut self) -> &String {
        if self.host.is_none() {
            self.host = Some(format!("{}:{}", self.address, self.port));
        }

        self.host.as_ref().unwrap()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            port: 7878,
            workers: 10,
            host: None,
        }
    }
}
