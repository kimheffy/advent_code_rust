use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Santa {
    x: isize,
    y: isize,
    prev_locations: HashSet<(isize, isize)>, // vector of tuples (x, y)
}

impl Santa {
    fn new(x: isize, y: isize) -> Santa {
        let mut locations: HashSet<(isize, isize)> = HashSet::new();
        locations.insert((x, y));

        Santa {
            x,
            y,
            prev_locations: locations,
        }
    }

    // Directions
    fn go_north(&mut self) {
        self.y += 1;
        self.update_direction();
    }

    fn go_south(&mut self) {
        self.y -= 1;
        self.update_direction();
    }

    fn go_east(&mut self) {
        self.x += 1;
        self.update_direction();
    }

    fn go_west(&mut self) {
        self.x -= 1;
        self.update_direction();
    }

    fn update_direction(&mut self) {
        if let None = self.prev_locations.get(&(self.x, self.y)) {
            self.prev_locations.insert((self.x, self.y));
        }
    }
}

fn main() {
    let mut santa: Santa = Santa::new(0, 0);
    let mut file: File = File::open("./house_course.txt").unwrap();
    let mut content: String = String::new();

    file.read_to_string(&mut content).unwrap();

    for c in content.chars() {
        read_symbols(&mut santa, c);
    }

    println!("How many house? {}", santa.prev_locations.len());
}

fn read_symbols(santa: &mut Santa, c: char) {
    match c {
        '^' => santa.go_north(),
        'v' => santa.go_south(),
        '>' => santa.go_east(),
        '<' => santa.go_west(),
        _ => {
            println!("Unknown symbol: {}", c);
        },
    }
}

#[test]
fn test_sample_one() {
    let mut santa: Santa = Santa::new(0, 0);
    let test_str = ">";

    for c in test_str.chars() {
        read_symbols(&mut santa, c);
    }

    println!("{:?}", &santa.prev_locations);
    assert_eq!(santa.prev_locations.len(), 2);
}

#[test]
fn test_sample_two() {
    let mut santa: Santa = Santa::new(0, 0);
    let test_str = "^>v<";

    for c in test_str.chars() {
        read_symbols(&mut santa, c);
    }

    println!("{:?}", &santa.prev_locations);
    assert_eq!(santa.prev_locations.len(), 4);
}

#[test]
fn test_sample_three() {
    let mut santa: Santa = Santa::new(0, 0);
    let test_str = "^v^v^v^v^v";

    for c in test_str.chars() {
        read_symbols(&mut santa, c);
    }

    println!("{:?}", &santa.prev_locations);
    assert_eq!(santa.prev_locations.len(), 2);
}

