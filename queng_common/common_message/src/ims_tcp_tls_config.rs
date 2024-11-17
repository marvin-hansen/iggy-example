use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TcpTlsConfig {
    // The  client address for the TCP transport
    tcp_server_address: String,
    // Flag to enable TLS for the TCP transport
    tcp_tls_enabled: bool,
    // The TLS domain for the TCP transport
    tcp_tls_domain: String,
    // The optional CA file for the TCP transport
    tcp_tls_ca_file: Option<String>,
}

impl TcpTlsConfig {
    /// Creates a new `TcpTlsConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `tcp_server_address` - The client address for the TCP transport.
    /// * `tcp_tls_enabled` - Flag to enable TLS for the TCP transport.
    /// * `tcp_tls_domain` - The optional TLS domain for the TCP transport.
    /// * `tcp_tls_ca_file` - The optional CA file for the TCP transport.
    ///
    /// # Returns
    ///
    /// A `TcpTlsConfig` instance.
    ///
    pub fn new(
        tcp_server_address: String,
        tcp_tls_enabled: bool,
        tcp_tls_domain: String,
        tcp_tls_ca_file: Option<String>,
    ) -> Self {
        Self {
            tcp_server_address,
            tcp_tls_enabled,
            tcp_tls_domain,
            tcp_tls_ca_file,
        }
    }
}

impl TcpTlsConfig {
    pub fn tcp_server_address(&self) -> &str {
        &self.tcp_server_address
    }

    pub fn tcp_tls_enabled(&self) -> bool {
        self.tcp_tls_enabled
    }

    pub fn tcp_tls_domain(&self) -> &str {
        &self.tcp_tls_domain
    }

    pub fn tcp_tls_ca_file(&self) -> &Option<String> {
        &self.tcp_tls_ca_file
    }
}

impl Default for TcpTlsConfig {
    fn default() -> Self {
        Self {
            tcp_server_address: "127.0.0.1:8090".to_string(),
            tcp_tls_enabled: false,
            tcp_tls_domain: "localhost".to_string(),
            tcp_tls_ca_file: None,
        }
    }
}

impl Display for TcpTlsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TcpTlsConfig: \n tcp_server_address: {}, \n tcp_tls_enabled: {}, \n  tcp_tls_domain: {}",
            self.tcp_server_address, self.tcp_tls_enabled, self.tcp_tls_domain
        )
    }
}