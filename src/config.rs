//! Application configuration types and parser.

use serde::Deserialize;
use std::net::SocketAddr;

/// A central type for configuration.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct App {
    /// Tcp frontends.
    #[serde(rename = "frontend")]
    pub frontend: Frontend,
}

/// A frontend where local traffic is received and then forward to backends.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Frontend {
    /// Local address where the frontend binds on.
    #[serde(rename = "bind_on")]
    pub local_address: SocketAddr,
    /// Collection of backends where traffic is forwarded to.
    #[serde(rename = "backend")]
    pub backends: Vec<Backend>,
}

/// A backend where local traffic is to be forwarded.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Backend {
    /// Target address for local traffic.
    #[serde(rename = "forward_to")]
    pub target_address: SocketAddr,
}

impl App {
    /// Parses a TOML-encoded configuration into [AppConfig].
    pub fn from_toml<S>(content: S) -> Result<App, toml::de::Error>
    where
        S: AsRef<str>,
    {
        toml::from_str(content.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[test]
    fn parses_a_simple_config() -> anyhow::Result<()> {
        // Pre-condition.
        let content = r#"
[frontend]
bind_on = "127.0.0.1:8000"

[[frontend.backend]]
forward_to = "127.0.0.1:9000"

[[frontend.backend]]
forward_to = "127.0.0.1:9001"
        "#;

        // Action.
        let config = App::from_toml(content)?;

        // Post-condition.
        assert_eq!(
            config,
            App {
                frontend: Frontend {
                    local_address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000).into(),
                    backends: vec![
                        Backend {
                            target_address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000)
                                .into(),
                        },
                        Backend {
                            target_address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9001)
                                .into(),
                        },
                    ],
                },
            }
        );

        Ok(())
    }
}
