#[cfg(test)]
mod tests_stefano {
    use crate::remove_duplicates_from_sorted_array_ii;

    #[test]
    fn test_1() {
        assert_eq!(remove_duplicates_from_sorted_array_ii::solutions::sol_stefano(&mut vec![1,1,1,2,2,3]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(remove_duplicates_from_sorted_array_ii::solutions::sol_stefano(&mut vec![0,0,1,1,1,1,2,3,3]), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(remove_duplicates_from_sorted_array_ii::solutions::sol_stefano(&mut vec![1,2,2]), 3);
    }
}