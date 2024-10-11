#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorted_array_1() {
        assert_eq!(MergeSortedArray::merge_stefano(&mut vec![1,2,3,0,0,0], 3, &mut vec![2,5,6], 3), vec![1,2,2,3,5,6]);
    }
}