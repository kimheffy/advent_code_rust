use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("list.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut num_list: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut set: HashSet<i32> = HashSet::new();

    for line in reader.lines() {
        let parse_int: i32 = line.unwrap().parse::<i32>().unwrap();
        num_list.push(parse_int);
    }

    for (i, x) in num_list.iter().enumerate() {
        for n in i + 1..=num_list.len() {
            let compliment: i32 = 2020 - x - num_list.get(n).unwrap();
            set.insert(compliment);
        }
    }

    println!("{:?}", set);
}
