#[cfg(test)]
mod test_file {
    extern crate passport;

    #[test]
    fn test_read_file() {
        passport::batch::read_file("test.txt");
        assert_eq!(2, 2);
    }
}

#[cfg(test)]
mod test_passport {
    extern crate passport;

    use passport::passport::Passport;

    #[test]
    fn test_create_new_passport() {
        let passport = Passport::new();

        assert_eq!(passport.byr, 0);
        assert_eq!(passport.iyr, 0);
        assert_eq!(passport.eyr, 0);
        assert_eq!(passport.hgt, "");
        assert_eq!(passport.hcl, "");
        assert_eq!(passport.ecl, "");
        assert_eq!(passport.pid, "");
        assert_eq!(passport.cid, "");
    }

    #[test]
    fn test_clear_passport() {
        let mut test_passport = Passport::new();

        // TODO: Stopped at getters and setters for passport;
        // Do I really need getters and setters?
        test_passport.byr = 1996;
        test_passport.iyr = 2021;
        test_passport.eyr = 2025;
        test_passport.hgt = "175.26cm".to_owned();
        test_passport.hcl = "black".to_owned();
        test_passport.ecl = "brown".to_owned();
        test_passport.pid = "123".to_owned();
        test_passport.cid = "A".to_owned();

        test_passport.clear();

        assert_eq!(test_passport.byr, 0);
        assert_eq!(test_passport.iyr, 0);
        assert_eq!(test_passport.eyr, 0);
        assert_eq!(test_passport.hgt, "");
        assert_eq!(test_passport.hcl, "");
        assert_eq!(test_passport.ecl, "");
        assert_eq!(test_passport.pid, "");
        assert_eq!(test_passport.cid, "");
    }

    #[test]
    fn test_passport_validate() {
        let mut test_passport = Passport::new();

        // TODO: Stopped at getters and setters for passport;
        // Do I really need getters and setters?
        test_passport.byr = 1996;
        test_passport.iyr = 2021;
        test_passport.eyr = 2025;
        test_passport.hgt = "175.26cm".to_owned();
        test_passport.hcl = "#866857".to_owned();
        test_passport.ecl = "brn".to_owned();
        test_passport.pid = "123".to_owned();
        test_passport.cid = "A".to_owned();

        assert_eq!(test_passport.validate_byr(), true);
        assert_eq!(test_passport.validate_iyr(), false);
        assert_eq!(test_passport.validate_hgt(), true);
        assert_eq!(test_passport.validate_hcl(), true);
        assert_eq!(test_passport.validate_ecl(), true);
        assert_eq!(test_passport.validate_pid(), false);
    }
}