//! Application configuration types and parser.

use derive_more::Display;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr};

/// A central type for configuration.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct AppConfig {
    /// Tcp frontends.
    #[serde(rename = "tcp_frontend")]
    tcp_frontends: HashMap<FrontendName, Frontend>,
}

/// A wrapper around a frontend name.
#[derive(Debug, Display, PartialEq, Eq, Hash, Deserialize)]
#[serde(transparent)]
pub struct FrontendName(String);

/// A frontend where local traffic is received and then forward to backends.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Frontend {
    /// Local address where the frontend binds on.
    #[serde(rename = "bind_on")]
    local_address: SocketAddr,
    /// Collection of backends where traffic is forwarded to.
    #[serde(rename = "backend")]
    backends: Vec<Backend>,
}

/// A backend where local traffic is to be forwarded.
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Backend {
    /// Target address for local traffic.
    #[serde(rename = "forward_to")]
    target_address: SocketAddr,
}

impl AppConfig {
    /// Parses a TOML-encoded configuration into [AppConfig].
    pub fn from_toml<S>(content: S) -> Result<AppConfig, toml::de::Error>
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
[tcp_frontend.experimental]
bind_on = "127.0.0.1:8000"

[[tcp_frontend.experimental.backend]]
forward_to = "127.0.0.1:9000"

[[tcp_frontend.experimental.backend]]
forward_to = "127.0.0.1:9001"
        "#;

        // Action.
        let config = AppConfig::from_toml(content)?;

        // Post-condition.
        assert_eq!(
            config,
            AppConfig {
                tcp_frontends: vec![(
                    FrontendName("experimental".into()),
                    Frontend {
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
                )]
                .into_iter()
                .collect(),
            }
        );

        Ok(())
    }
}
