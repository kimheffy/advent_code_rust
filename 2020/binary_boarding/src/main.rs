extern crate f_reader;
use std::io::BufRead;

use binary_boarding::logic::{run, compute};


fn main() {
    let read = f_reader::read("seats.txt");
    let mut highest: i32 = 0;

    for lines in read.lines() {
        if let Ok(line) = lines {
            let result = compute(run(&line));
            if result > highest {
                highest = result;
            }
        }
    }

    println!("Highest: {}", highest);
}

#[cfg(test)]
mod tests {
    use binary_boarding::logic::{run, compute};

    #[test]
    fn example_1() {
        let ans: (i32, i32) = run("BFFFBBFRRR");
        assert_eq!((70, 7), ans);
        assert_eq!(567, compute(ans));
    }

    #[test]
    fn example_2() {
        let ans: (i32, i32) = run("FFFBBBFRRR");
        assert_eq!((14, 7), ans);
        assert_eq!(119, compute(ans));
    }

    #[test]
    fn example_3() {
        let ans: (i32, i32) = run("BBFFBBFRLL");
        assert_eq!((102, 4), ans);
        assert_eq!(820, compute(ans));
    }
}
