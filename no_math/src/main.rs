use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn surface_area(v: &Vec<usize>) -> usize {
    // length, width, height
    //(2*self.length*self.width) + (2*self.width*self.height) + (2*self.height*self.length)
    (2*&v[0]*&v[1]) + (2*&v[1]*&v[2]) + (2*&v[2]*&v[0])
}

fn find_smallest_area(v: &Vec<usize>) -> usize {
    let mut smallest: (usize, usize) = (0, usize::MAX);
    let mut second_smallest: usize = usize::MAX;

    for (i, &n) in v.iter().enumerate() {
        if n < smallest.1 {
            smallest.0 = i;
            smallest.1 = n;
        }
    }

    for (j, &m) in v.iter().enumerate() {
        if m < second_smallest && m >= smallest.1 && j != smallest.0 {
            second_smallest = m;
        }
    }

    smallest.1 * second_smallest
}


fn main() {
    // Read the elves_list file
    let file = File::open("elves_list.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut total_amount: usize = 0;

    for line in buf_reader.lines() {
        if let Ok(str) = &line {
            let dimensions: Vec<usize>  = str.split("x").map(|c| c.parse::<usize>().unwrap()).collect();
            total_amount += surface_area(&dimensions) + find_smallest_area(&dimensions);
        }
    }

    println!("Elves would need: {}", total_amount);
}

