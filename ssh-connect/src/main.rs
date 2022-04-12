use std::cmp::min;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use adapter::argument_parser;
use adapter::file_manager::write_file;
use adapter::server_stream::connect;

fn main() {
    let variables = argument_parser::parse();
    let (download_path, contents) = connect(&variables);
    write_file(&download_path, &contents);

    // let mut downloaded = 0;
    // let total_size = 23123;
    //
    // let pb = ProgressBar::new(total_size);
    // pb.set_style(ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //     .progress_chars("#>-"));
    //
    // while downloaded < total_size {
    //     let new = min(downloaded + 2232, total_size);
    //     downloaded = new;
    //     pb.set_position(new);
    //     thread::sleep(Duration::from_millis(12));
    // }
    //
    // pb.finish_with_message("downloaded");
}

