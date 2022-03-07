use adapter::argument_parser;
use adapter::file_manager::write_file;
use adapter::server_stream::connect;

fn main() {
    let variables = argument_parser::parse();
    let (download_path, contents) = connect(&variables);
    write_file(&download_path, &contents);
}

