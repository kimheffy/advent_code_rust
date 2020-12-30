mod part2;

extern crate f_reader;

use f_reader::read;
use std::io::BufRead;

fn main() {
    let reader = read("list.txt");
    let mut counter = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            if true == part2::part2(&l) {
                counter += 1;
            }
        }
    }

    println!("Amount: {}", counter);
}

fn parse_passwrd(s: &str) -> bool {
    // TODO: Deconstruct this vec;
    let split: Vec<&str> = s.split(":").map(|s| s.trim()).collect();
    let policy = split[0];
    let passwrd = split[1];

    let parse_policy: Vec<&str> = policy.split(" ").collect();
    let parse_boundries: Vec<u32> = parse_policy[0].split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let min = parse_boundries[0];
    let max = parse_boundries[1];
    let c = parse_policy[1].parse::<char>().unwrap();

    return valid_policy(min, max, c, &passwrd);
}

fn valid_policy(low: u32, high: u32, c: char, passwrd: &str) -> bool {
    let mut counter: u32 = 0;

    for ch in passwrd.chars() {
        if c == ch {
            counter += 1;
        }
    }

    return low <= counter && counter <= high;
}

#[cfg(test)]
mod test {
    use crate::parse_passwrd;

    #[test]
    fn example1() {
        let actual = parse_passwrd("1-3 a: abcde");
        let expected = true;

        assert_eq!(expected, actual);
    }

    #[test]
    fn example2() {
        let actual = parse_passwrd("1-3 b: cdefg");
        let expected = false;

        assert_eq!(expected, actual);
    }

    #[test]
    fn example3() {
        let actual = parse_passwrd("2-9 c: ccccccccc");
        let expected = true;

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod test2 {
    use crate::part2::part2;

    #[test]
    fn example1() {
        let expect = true;
        let actual = part2("1-3 a: abcde");
        assert_eq!(expect, actual);
    }

    #[test]
    fn example2() {
        let expect = false;
        let actual = part2("1-3 b: cdefg");
        assert_eq!(expect, actual);
    }

    #[test]
    fn example3() {
        let expect = false;
        let actual = part2("2-9 c: ccccccccc");
        assert_eq!(expect, actual);
    }
}
