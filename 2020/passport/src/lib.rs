pub mod passport {
    /// A Passport is represented here.
    /// byr - birth year
    /// iyr - issue year
    /// eyr - expiration year
    /// hgt - height
    /// hcl - hair color
    /// ecl - eye color
    /// pid - passport id
    /// cid - country id
    #[derive(Debug)]
    pub struct Passport {
        pub byr: usize,
        pub iyr: usize,
        pub eyr: usize,
        pub hgt: String,
        pub hcl: String,
        pub ecl: String,
        pub pid: String,
        pub cid: String
    }

    impl Passport {
        pub fn set_byr(&mut self, value: usize) {
            self.byr = value;
        }

        pub fn set_iyr(&mut self, value: usize) {
            self.iyr = value;
        }

        pub fn set_eyr(&mut self, value: usize) {
            self.eyr = value;
        }

        pub fn set_hgt(&mut self, value: &str) {
            self.hgt = value.to_owned();
        }

        pub fn set_hcl(&mut self, value: &str) {
            self.hcl = value.to_owned();
        }

        pub fn set_ecl(&mut self, value: &str) {
            self.ecl = value.to_owned();
        }

        pub fn set_pid(&mut self, value: &str) {
            self.pid = value.to_owned();
        }

        pub fn set_cid(&mut self, value: &str) {
            self.cid = value.to_owned();
        }
    }

    impl Passport {
        /// Creates a new passport struct;
        pub fn new() -> Passport {
            Passport {
                byr: 0,
                iyr: 0,
                eyr: 0,
                hgt: String::new(),
                hcl: String::new(),
                ecl: String::new(),
                pid: String::new(),
                cid: String::new()
            }
        }

        /// Clears the existing passport;
        pub fn clear(&mut self) {
            self.byr = 0;
            self.iyr = 0;
            self.eyr = 0;
            self.hgt = String::new();
            self.hcl = String::new();
            self.ecl = String::new();
            self.pid = String::new();
            self.cid = String::new();
        }

        pub fn insert(&mut self, key: &str, value: usize) {
            match key {
                "byr" => self.set_byr(value),
                "iyr" => self.set_iyr(value),
                "eyr" => self.set_eyr(value),
                _ => panic!("Couldn't find {}", key),
            }
        }

        pub fn insert_str(&mut self, key: &str, value: &str) {
            match key {
                "hgt" => self.set_hgt(value),
                "hcl" => self.set_hcl(value),
                "ecl" => self.set_ecl(value),
                "pid" => self.set_pid(value),
                "cid" => self.set_cid(value),
                _ => panic!("Couldn't find {}", key),
            }
        }
    }

    impl Passport {
        pub fn validate_byr(&self) -> bool {
            1920 <= self.byr && self.byr <= 2002
        }

        pub fn validate_iyr(&self) -> bool {
            2010 <= self.iyr && self.iyr <= 2020
        }

        pub fn validate_eyr(&self) -> bool {
            2020 <= self.eyr && self.eyr <= 2030
        }

        // TODO: Refactor
        pub fn validate_hgt(&self) -> bool {
            if !(self.hgt.contains("cm") || self.hgt.contains("in")) {
                return false;
            } else {
                if self.hgt.contains("cm") {
                    let measure: Vec<&str> = self.hgt.split("cm").collect();
                    let data: f32 = measure[0].parse::<f32>().unwrap_or_else(|err| {
                        panic!("Parse Float error: {}", err);
                    });

                    return 150.0 <= data && data <= 193.0;
                } else {
                    let measure: Vec<&str> = self.hgt.split("in").collect();
                    let data: f32 = measure[0].parse::<f32>().unwrap_or_else(|err| {
                        panic!("Parse Float error: {}", err);
                    });

                    return 59.0 <= data && data <= 76.0;
                }
            }
        }

        fn range_check(s: &str, low: char, high: char) -> bool {
            let mut result: bool = false;
            for strin in s.chars() {
                if low <= strin && strin <= high {
                    result = true;
                }
            }

            result
        }

        pub fn validate_hcl(&self) -> bool {
            if self.hcl.contains("#") {
                let hex_container: Vec<&str> = self.hcl.split("#").collect();
                let hex: &str = hex_container[1];
                return hex.len() == 6 && (Self::range_check(hex, '0', '9') || Self::range_check(hex, 'a', 'z'));
            } else {
                return false;
            }
        }

        pub fn validate_ecl(&self) -> bool {
            self.ecl == "amb" || self.ecl == "blu" || self.ecl == "brn" || self.ecl == "gry" || self.ecl == "grn" || self.ecl == "hzl" || self.ecl == "oth"
        }

        pub fn validate_pid(&self) -> bool {
            self.pid.len() == 9
        }

        pub fn validate_all(&self) -> bool {
            self.validate_byr() && self.validate_iyr() && self.validate_eyr()
            && self.validate_hgt() && self.validate_hcl() && self.validate_ecl() &&
                self.validate_pid()
        }
    }
}

pub mod batch {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    use super::passport::Passport;

    pub fn read_file(path: &str) {
        let file;

        // check if the path exist in the root;
        if let Ok(f) = File::open(path) {
            file = f;
        } else {
            panic!("Cannot open {}", path);
        }

        // read the file;
        let buf_reader = BufReader::new(file);
        // create a new passport struct;
        let mut passport = Passport::new();
        let mut total = 0;

        for line in buf_reader.lines() {
            if let Ok(data) = line {
                // Continued data
                if data.len() != 0 {
                    // have to start parsing the date with my batch module;
                    parse_line(&data, &mut passport);
                } else {
                    println!("{:?}", &passport);
                    if passport.validate_all() {
                        total += 1;
                    }
                    // clear the previous passport;
                    passport.clear();
                }
            }
        }
        println!("{:?}", &passport);
        if passport.validate_all() {
            total += 1;
        }

        println!("Total {}", total);
    }

    /// reading in a line and parsing each field to match passport;
    pub fn parse_line(line: &str, ref_passport: &mut Passport) {
        for l_split in line.split(" ") {
            let pair: Vec<&str> = l_split.split(":").collect();
            let key: &str = pair[0];
            if key == "byr" || key == "eyr" || key == "iyr" {
                let value = pair[1].parse::<usize>().unwrap();
                ref_passport.insert(key, value);
            } else {
                let value = pair[1];
                ref_passport.insert_str(key, value);
            }
        }
    }
}