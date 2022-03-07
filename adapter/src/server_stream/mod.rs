use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use rpassword::read_password;
use ssh2::Session;
use configuration::app_configuration::{AppConfig, ServerData};
use crate::argument_parser::EnvironmentVariables;

pub fn connect(variables: &EnvironmentVariables) -> (String, Vec<u8>) {

    // configuration
    let app_config = AppConfig::new().unwrap();
    let configuration: &ServerData = app_config.config(&variables.server);


    // server connection
    let tcp = TcpStream::connect(configuration.ip().to_string()).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // authenticate
    let password = get_password();
    sess.userauth_password(configuration.username(), &password)
        .unwrap();

    // download file
    let path: String = configuration.path().to_string();
    let file_to_download = path + &variables.filename;
    let (mut remote_file, stat) = sess.scp_recv(Path::new(&file_to_download)).unwrap();

    println!("Download {} file size: {}", variables.filename, stat.size());
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    println!("File {} downloaded", variables.filename);

    let download_path: String = app_config.download().path().to_owned() + &variables.filename;

    println!("{}", download_path);

    (download_path, contents)
}

fn get_password() -> String {
    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    read_password().unwrap()
}
