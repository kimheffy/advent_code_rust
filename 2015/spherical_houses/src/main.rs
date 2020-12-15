use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point {
            x,
            y,
        }
    }

    // Directions
    fn go_north(&mut self) {
        self.y += 1;
    }

    fn go_south(&mut self) {
        self.y -= 1;
    }

    fn go_east(&mut self) {
        self.x += 1;
    }

    fn go_west(&mut self) {
        self.x -= 1;
    }

    fn get_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    let mut set: HashSet<Point> = HashSet::new();
    let mut santa: Point = Point::new(0, 0);
    let mut robo_santa: Point = Point::new(0, 0);

    let mut santa_turn: bool = true;
    let mut robo_turn: bool = false;

    set.insert(Point{x: 0, y:0});

    // TODO: Change this file;
    let mut file: File = File::open("./house_course.txt").unwrap();
    let mut content: String = String::new();

    file.read_to_string(&mut content).unwrap();

    for c in content.chars() {
        if santa_turn {
            read_symbols(&mut santa, c, &mut set);
            santa_turn = false;
            robo_turn = true;
        } else if robo_turn {
            read_symbols(&mut robo_santa, c, &mut set);
            robo_turn = false;
            santa_turn = true;
        }
    }

    println!("How many house? {}", set.len());
}

fn read_symbols(santa: &mut Point, c: char, set: &mut HashSet<Point>) {
    match c {
        '^' => santa.go_north(),
        'v' => santa.go_south(),
        '>' => santa.go_east(),
        '<' => santa.go_west(),
        _ => {
            println!("Unknown symbol: {}", c);
        },
    }

    println!("{:?}", santa.get_point());
    set.insert(santa.get_point());
}

/*
#[test]
fn test_sample_one() {
    let mut santa: Point  = Santa::new(0, 0);
    let mut robo_santa: Santa = Santa::new(0, 0);
    let test_str = "^v";

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
*/
