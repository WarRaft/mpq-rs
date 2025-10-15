use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use mpq_rs::Archive;

#[test]
fn print_w3x_file_list() {
    let map_path = Path::new("test-data/maps/(2)BootyBay.w3m");
    let file = File::open(map_path).expect("failed to open test map");
    let reader = BufReader::new(file);
    let mut archive = Archive::open(reader).expect("failed to parse map as MPQ archive");

    let files = archive
        .files()
        .expect("map should contain a (listfile) with file names");
    assert!(
        !files.is_empty(),
        "expected map listfile to contain at least one entry"
    );

    for entry in files {
        println!("{entry}");
    }
}
