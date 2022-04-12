use std::env;

pub struct EnvironmentVariables {
    pub server: String,
    pub filename: String,
}

impl EnvironmentVariables {
    pub fn new(server: &str, filename: &str) -> Self {
        EnvironmentVariables { server: String::from(server), filename: String::from(filename) }
    }
}


pub fn parse() -> EnvironmentVariables {
    let args: Vec<String> = env::args().collect();
    let server_env = &args[1];
    let filename = &args[2];
    EnvironmentVariables::new(server_env, filename)
}
