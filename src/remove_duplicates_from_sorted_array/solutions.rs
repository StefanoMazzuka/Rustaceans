pub fn sol_stefano(nums: &mut Vec<i32>) -> i32 {
    if (nums.len() == 1) {
        return 1;
    }
    if (nums.len() > 1) {
        let mut j = 0 as usize;
    
        for i in 1..nums.len() {
            if nums[i] > nums[j] {
                j += 1;
                nums[j] = nums[i];
            }
        }

        return j as i32 + 1;
    }

    return 0;
}