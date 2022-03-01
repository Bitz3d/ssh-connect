use std::cmp::min;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::os::macos::raw::stat;
use std::path::Path;

use rpassword::read_password;
use ssh2::Session;

use app_configuration::AppConfig;

use crate::app_configuration::ConnectionData;

mod app_configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_env = &args[1];
    let filename = &args[2];

    let settings = AppConfig::new().unwrap();

    let configuration: &ConnectionData = settings.get_env_variables(server_env);

    // Connect to the local SSH server
    let tcp = TcpStream::connect(configuration.ip().to_string()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let password = get_password();

    sess.userauth_password(configuration.username(), &password).unwrap();

    let path: String = configuration.path().to_string();
    let file_to_download = path + filename;


    let (mut remote_file, stat) = sess.scp_recv(Path::new(&file_to_download)).unwrap();


    println!("Download {} file size: {}", filename, stat.size());
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    println!("File {} downloaded", filename);

    let download_path: String = settings.download().path().to_owned() + filename;

    println!("{}", download_path);

    std::fs::write(download_path, &contents).unwrap();
}

fn get_password() -> String {
    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    password
}
