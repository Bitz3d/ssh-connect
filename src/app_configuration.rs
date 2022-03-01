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
    path: String

}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    prod: ServerData,
    uat21: ServerData,
    uat22: ServerData,
    download: Download
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("./config.toml"))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }

    fn prod(&self) -> &ServerData {
        &self.prod
    }
    fn uat21(&self) -> &ServerData {
        &self.uat21
    }
    fn uat22(&self) -> &ServerData {
        &self.uat22
    }



    pub fn get_env_variables(&self, env: &String) -> &ServerData {
        if "uat21" == env {
            return self.uat21();
        }
        if "uat22" == env {
            return self.uat22();
        }
        return self.prod();
    }

    pub fn download(&self) -> &Download {
        &self.download
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
