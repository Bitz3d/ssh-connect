pub fn write_file(download_path: &str, contents: &[u8]){
    std::fs::write(download_path, &contents).unwrap();
}
