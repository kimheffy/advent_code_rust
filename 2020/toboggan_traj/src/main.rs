extern crate f_reader;

use f_reader::read;

use std::io::BufRead;
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

fn main() {
    // GOAL: Count the number of trees;
    let reader = read("final.txt");

    let mut pos: usize = 0;
    let mut trees: usize = 0;
    let tree: u8 = 35;

    for line in reader.lines() {
        let str = line.unwrap();
        // // check the position in s string
        let str_buf = &str.as_bytes();
        println!("pos: {}\tbuf len: {}", pos, &str_buf.len());
        if pos >= str_buf.len() {
            pos = pos - str_buf.len();
        }
        let c = &str_buf[pos];
        if c.eq(&tree) {
            trees += 1;
        }
        pos += 3;
    }

    println!("Number of trees: {}", trees);
}
