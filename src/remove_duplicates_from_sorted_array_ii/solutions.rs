pub fn sol_stefano(nums: &mut Vec<i32>) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => return 1,
        _=> {
            let mut j = 0 as usize;
            let mut steps = if nums[0] == nums[1] { 1 } else { 0 };
    
            for i in 1..nums.len() {
                if nums[i] > nums[j] {
                    j += 1;
                    nums[j] = nums[i];
                    steps = 0;
                } else if steps == 1 {
                    j += 1;
                    nums[j] = nums[i];
                }
                steps += 1;
            }
    
            return j as i32 + 1;
        }
    }
}