
fn cal_floors(s: &str) -> isize {
    let mut floor: isize = 0;

    for c in s.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(cal_floors("(())"), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(cal_floors("((("), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(cal_floors("())"), -1);
    }

    #[test]
    fn test4() {
        assert_eq!(cal_floors(")))"), -3);
    }
}
