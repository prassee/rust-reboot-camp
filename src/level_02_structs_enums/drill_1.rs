#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
    verbose: bool,
}

#[derive(Debug, PartialEq)]
pub struct Meters(pub f64);

#[derive(Debug, PartialEq)]
pub struct HealthCheck;

impl ServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            max_connections: 100,
            verbose: false,
        }
    }

    pub fn with_port(&self, port: u16) -> Self {
        Self {
            host: self.host.clone(),
            port,
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_server_config() {
        let host = "127.0.0.1".to_string();
        let port = 3306;
        let server_config = ServerConfig::new(host, port);
        assert_eq!(server_config.max_connections, 100);
        assert!(!server_config.verbose);
        assert_eq!(server_config.host, "127.0.0.1".to_string());
        assert_eq!(server_config.port, 3306);
    }

    #[test]
    fn test_server_config_from_base() {
        let base = ServerConfig::new("127.0.0.1".to_string(), 3306);
        let derived = ServerConfig { port: 9000, ..base };
        assert_eq!(base.port, 3306); // passes 
        assert!(!base.verbose); // passes 
        assert_eq!(base.max_connections, 100); // passes 

        // assert_eq!(derived.host, base.host); fails
        assert_eq!(derived.port, 9000); // passes 
        assert!(!derived.verbose); // passes 
        assert_eq!(derived.max_connections, base.max_connections); // passes 

        // println!("{:?}", base); // failed due to not implementing the copy trait
        // println!("{:?}", base.host); // failed  - due to not implementing the copy trait
    }

    #[test]
    fn test_backup_server_config() {
        let host = "127.0.0.1".to_string();
        let port = 3306;
        let server_config = ServerConfig::new(host, port);
        let backup_server = server_config.with_port(3307);
        assert_eq!(backup_server.max_connections, server_config.max_connections);
        assert_eq!(backup_server.verbose, server_config.verbose);
        assert_eq!(backup_server.host, server_config.host);
        assert_eq!(backup_server.port, 3307);
    }

    #[test]
    fn test_read_meters() {
        let Meters(c) = Meters(40.0);
        assert_eq!(c, 40.0);
        let cpu_load = Meters(40.0);
        assert_eq!(cpu_load.0, 40.0);
    }

    #[test]
    fn test_hc_equality() {
        let hc1 = HealthCheck;
        let hc2 = HealthCheck;
        assert_eq!(hc1, hc2);
    }
}
