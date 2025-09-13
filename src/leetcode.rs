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

/*
    The count-and-say sequence is a sequence of digit strings defined by the recursive formula:

    countAndSay(1) = "1"
    countAndSay(n) is the run-length encoding of countAndSay(n - 1).
    Run-length encoding (RLE) is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times)
    with the concatenation of the character and the number marking the count of the characters (length of the run).
    For example, to compress the string "3322251" we replace "33" with "23", replace "222" with "32", replace "5" with "15" and replace "1" with "11".
    Thus the compressed string becomes "23321511".

    Given a positive integer n, return the nth element of the count-and-say sequence.
*/
#[allow(unused)]
pub fn count_and_say(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }

    let prev: String = count_and_say(n - 1);
    let mut result = String::new();
    let chars: Vec<char> = prev.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 1;
        while i + 1 < chars.len() && chars[i] == chars[i + 1] {
            i += 1;
            count += 1;
        }
        result.push_str(&count.to_string());
        result.push(chars[i]);
        i += 1;
    }
    result
}
