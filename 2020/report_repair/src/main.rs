use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("list.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut num_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let parse_int: i32 = line.unwrap().parse::<i32>().unwrap();
        num_list.push(parse_int);
    }

    // Triple loop
    for i in 0..num_list.len() {
        for j in i+1..num_list.len() {
            for k in j+1..num_list.len() {
                if num_list[i] + num_list[j] + num_list[k] == 2020 {
                    let product = num_list[i] * num_list[j] * num_list[k];
                    println!("The product is: {}", product);
                }
            }
        }
    }
}
