extern crate f_reader;

use f_reader::read;

use std::io::{BufRead, BufReader};
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::ops::Index;
use std::borrow::Borrow;

fn extend_file_by(n: u32) {
    let reader = read("sample.txt");
    let mut buffer = File::create("final.txt").unwrap();

    for line in reader.lines() {
        let s: String = line.unwrap();
        for _ in 0..5 {
            buffer.write(s.as_bytes());
        }
        buffer.write(b"\n");
    }
}

struct Sled {
    pos: usize,
    down: usize,
    trees: i32,
}

impl Sled {
    fn new() -> Sled {
        Sled {
            pos: 0,
            down: 0,
            trees: 0
        }
    }

    fn slope(&mut self, right: usize, down: usize, line: &str) {
        let tree_byte: u8 = 35;

        let str_buf = &line.as_bytes();
        if self.pos >= str_buf.len() {
            self.pos = self.pos - str_buf.len();
        }

        let cur_char = &str_buf[self.pos];
        if cur_char.eq(&tree_byte) {
            self.trees += 1;
        }

        self.pos += right;
    }

    fn special_slope(&mut self, right: usize, down: usize, line: &str, cur_line: usize) {
        let tree_byte: u8 = 35;

        if cur_line != 0 && cur_line % down == 0 {
            let str_buf = &line.as_bytes();
            if self.pos >= str_buf.len() {
                self.pos = self.pos - str_buf.len();
            }

            let cur_char = &str_buf[self.pos];
            if cur_char.eq(&tree_byte) {
                self.trees += 1;
            }

            self.pos += right;
        }
    }

    fn set_right(&mut self, right: usize) {
        self.pos = right;
    }
}

fn main() {
    // extend_file_by(5);

    let reader = read("final.txt");

    let mut sled1: Sled = Sled::new();
    let mut sled2: Sled = Sled::new();
    let mut sled3: Sled = Sled::new();
    let mut sled4: Sled = Sled::new();
    let mut sled5: Sled = Sled::new();

    sled5.set_right(1);

    let mut rules: Vec<Sled> = vec![sled1, sled2, sled3, sled4, sled5];

    let mut cur_line: usize = 0;

    for line in reader.lines() {
        let str: String = line.unwrap();
        rules[0].slope(1, 1, &str);
        rules[1].slope(3, 1, &str);
        rules[2].slope(5, 1, &str);
        rules[3].slope(7, 1, &str);
        rules[4].special_slope(1, 2, &str, cur_line);
        cur_line += 1;
    }

    let mut product: i64 = 1;

    for (i, s) in rules.iter().enumerate() {
        println!("Index: {}\tTrees: {}", i, s.trees);
        product *= s.trees as i64;
    }

    println!("Product: {}", product);
    println!("{}", 1 / 2);
}
