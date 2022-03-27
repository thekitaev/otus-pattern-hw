#![allow(dead_code)]

#[derive(Default, PartialEq, Eq, Debug)]
struct Server {
    host: String,
    port: u16,
}

struct ServerBuilder {
    inner: Server,
}

impl ServerBuilder {
    fn new() -> Self {
        Self {
            inner: Server::default(),
        }
    }
    fn with_host(mut self, host: &str) -> Self {
        self.inner.host = host.to_string();
        self
    }
    fn with_port(mut self, port: u16) -> Self {
        self.inner.port = port;
        self
    }
    fn build(self) -> Result<Server, String> {
        if self.inner.host.is_empty() {
            Err("host not provided".to_string())
        } else if self.inner.port == 0 {
            Err("port not set".to_string())
        } else {
            Ok(self.inner)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Server, ServerBuilder};

    #[test]
    fn test_builder_ok() {
        let want = Server{host: "example.com".to_string(), port: 2250};
        let have = ServerBuilder::new().with_host("example.com").with_port(2250).build().unwrap();

        assert_eq!(have, want);
    }

    #[test]
    fn test_builder_err() {
        assert_eq!(ServerBuilder::new().build().unwrap_err(), "host not provided");
        assert_eq!(ServerBuilder::new().with_host("host").build().unwrap_err(), "port not set");
    }
}
