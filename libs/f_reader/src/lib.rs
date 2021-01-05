use std::path::Path;
use std::fs::File;
use std::io::BufReader;

pub fn read(filename: &str) -> BufReader<File>{
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    BufReader::new(file)
}
