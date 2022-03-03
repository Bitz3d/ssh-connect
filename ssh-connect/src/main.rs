use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

use rpassword::read_password;
use ssh2::Session;
use configuration::app_configuration::{AppConfig, ServerData};


fn main() {
    let args: Vec<String> = env::args().collect();
    let server_env = &args[1];
    let filename = &args[2];

    let app_config = AppConfig::new().unwrap();

    let configuration: &ServerData = app_config.config(server_env);

    // Connect to the local SSH server
    let tcp = TcpStream::connect(configuration.ip().to_string()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let password = get_password();

    sess.userauth_password(configuration.username(), &password)
        .unwrap();
    //
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

    let download_path: String = app_config.download().path().to_owned() + filename;

    println!("{}", download_path);

    std::fs::write(download_path, &contents).unwrap();
}

fn get_password() -> String {
    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    read_password().unwrap()
}
