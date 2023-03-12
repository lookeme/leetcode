pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0 as i32;
    }
    let mut prev = 0;
    for i in 1..nums.len() {
        if nums[prev] != nums[i] {
            prev += 1;
            nums[prev] = nums[i];
        }
    }
    return (prev + 1) as i32;
}
