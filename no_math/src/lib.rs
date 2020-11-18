
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let box1 = Box {
            length: 2,
            width: 3,
            height: 4,
        };

        assert_eq!(box1.surface_area(), 52);
        assert_eq!(box1.area_smallest_side(), 6);
        assert_eq!(box1.total_package(), 58);
    }

    #[test]
    fn test2() {
        let box1 = Box {
            length: 1,
            width: 1,
            height: 10,
        };

        assert_eq!(box1.surface_area(), 42);
        assert_eq!(box1.area_smallest_side(), 1);
        assert_eq!(box1.total_package(), 43);
    }
}
