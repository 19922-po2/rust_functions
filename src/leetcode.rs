//use std::collections::HashMap;
/*
    Given a 0-indexed integer array nums of length n and an integer k,
    return the number of pairs (i, j) where 0 <= i < j < n, such that nums[i] == nums[j] and (i * j) is divisible by k.
*/
#[allow(unused)]
pub fn count_pairs() {
    //let nums: Vec<i32> = vec![3, 1, 2, 2, 2, 1, 3];
    //let k: i32 = 2;
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let k: i32 = 1;
    let mut result: i32 = 0;

    fn check_conditions(i: usize, j: usize, k: usize, nums: &Vec<i32>) -> bool {
        return nums[i] == nums[j] && (i * j) % k == 0;
    }

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if check_conditions(i, j, k as usize, &nums) {
                result += 1
            };
        }
    }

    println!("{:?}", result);
}
