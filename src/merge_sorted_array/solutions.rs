pub fn sol_stefano(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    let mut i: i32 = m - 1;
    let mut j: i32 = n - 1;

    for pos in (0..(nums1.len())).rev() {
        if i >= 0 && (j < 0 || nums1[i as usize] > nums2[j as usize]) {
            nums1[pos] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[pos] = nums2[j as usize];
            j -= 1;
        }
    }

    return nums1.to_vec();
}

pub fn sol_javi(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
    return vec![0]
}