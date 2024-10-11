#[cfg(test)]
mod tests_stefano {
    use crate::merge_sorted_array;

    #[test]
    fn test_1() {
        assert_eq!(merge_sorted_array::solutions::sol_stefano(&mut vec![1,2,3,0,0,0], 3, &mut vec![2,5,6], 3), vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(merge_sorted_array::solutions::sol_stefano(&mut vec![1], 1, &mut vec![], 0), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(merge_sorted_array::solutions::sol_stefano(&mut vec![0], 0, &mut vec![1], 1), vec![1]);
    }
}

mod tests_javi {
    use crate::merge_sorted_array;

    #[test]
    fn test_1() {
        assert_eq!(merge_sorted_array::solutions::sol_javi(&mut vec![1,2,3,0,0,0], 3, &mut vec![2,5,6], 3), vec![1,2,2,3,5,6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(merge_sorted_array::solutions::sol_javi(&mut vec![1], 1, &mut vec![], 0), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(merge_sorted_array::solutions::sol_javi(&mut vec![0], 0, &mut vec![1], 1), vec![1]);
    }
}