pub mod app_configuration{
    use std::collections::HashMap;
    use std::path::Path;

    use config::{Config, ConfigError, Environment, File};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    pub struct ServerData {
        username: String,
        path: String,
        ip: String,
    }

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    pub struct Download {
        path: String,
    }

    #[derive(Debug, Deserialize)]
    #[allow(unused)]
    pub struct AppConfig {
        servers_data: HashMap<String, ServerData>,
        download: Download,
    }

    impl AppConfig {
        pub fn new() -> Result<Self, ConfigError> {
            let s = Config::builder()
                // Start off by merging in the "default" configuration file
                .add_source(File::from(Path::new("./config.yml")))
                .add_source(Environment::with_prefix("app"))
                .build()?;

            // You can deserialize (and thus freeze) the entire configuration as
            s.try_deserialize()
        }

        pub fn download(&self) -> &Download {
            &self.download
        }

        pub fn config(&self, server_env: &str) -> &ServerData {
            self.servers_data.get(server_env).unwrap()
        }
    }

    impl ServerData {
        pub fn username(&self) -> &str {
            &self.username
        }
        pub fn path(&self) -> &str {
            &self.path
        }
        pub fn ip(&self) -> &str {
            &self.ip
        }
    }

    impl Download {
        pub fn path(&self) -> &str {
            &self.path
        }
    }
}
