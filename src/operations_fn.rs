use std::collections::HashMap;

/*
    You are given two integers num1 and num2.
    In one operation, you can choose integer i in the range [0, 60] and subtract 2i + num2 from num1.
    Return the integer denoting the minimum number of operations needed to make num1 equal to 0.
    If it is impossible to make num1 equal to 0, return -1.
*/
#[allow(unused)]
pub fn min_op_to_zero() {
    let num1 = 5;
    let num2 = 7;
    let mut res = 0;

    for k in 1..=60 {
        let s = num1 - k as i64 * num2;
        if s < 0 {
            continue;
        }
        let pop = s.count_ones() as i64;
        if pop <= k as i64 && k as i64 <= s {
            res = k as i32;
            println!("Result: {}", res);
            break;
        }
    }
    if res <= 0 {
        println!("Result: {}", -1);
    }
}

/*
    No-Zero integer is a positive integer that does not contain any 0 in its decimal representation.
    Given an integer n, return a list of two integers [a, b] where:
    a and b are No-Zero integers.
    a + b = n
    The test cases are generated so that there is at least one valid solution. If there are many valid solutions, you can return any of them.
*/
#[allow(unused)]
pub fn convert_to_sum_of_two_no_zero() {
    let n = 1000;
    for x in 1..n {
        let res = n - x;
        if (x + res) == n && !res.to_string().contains('0') && !x.to_string().contains('0') {
            println!("{:?}", vec!((x), (res)));
            break;
        }
    }
}

/*
    You are given an integer n.
    We need to group the numbers from 1 to n according to the sum of its digits. For example, the numbers 14 and 5 belong to the same group, whereas 13 and 3 belong to different groups.
    Return the number of groups that have the largest size, i.e. the maximum number of elements.
*/
#[allow(unused)]
pub fn count_largest_group() {
    let n = 2;

    let mut digit_sum_groups: HashMap<u32, u32> = HashMap::new();

    // Group numbers by the sum of their digits
    for num in 1..=n {
        let digit_sum = sum_of_digits(num);
        let count = digit_sum_groups.entry(digit_sum).or_insert(0);
        *count += 1;
    }

    // Find the largest group size
    let max_size = *digit_sum_groups.values().max().unwrap();

    fn sum_of_digits(x: u32) -> u32 {
        let mut sum = 0;
        let mut num = x;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
    // Count how many groups have the largest size
    println!(
        "{}",
        digit_sum_groups.values().filter(|&&count| count == max_size).count() as u32
    )
}

/*
    Given an integer n, return any array containing n unique integers such that they add up to 0.
*/
#[allow(unused)]
pub fn sum_zero() {
    let n = 7;
    let mut res: Vec<i32> = Vec::new();

    if n % 2 == 0 {
        for x in 1..n / 2 + 1 {
            res.push(x);
            res.push(-x);
        }
    } else {
        res.push(0);
        for x in 1..n / 2 + 1 {
            res.push(x);
            res.push(-x);
        }
    }
    println!("{:?}", res);
}

/*
    Given an array nums of integers, return how many of them contain an even number of digits.
*/
#[allow(unused)]
pub fn find_numbers() {
    let nums = [555,901,482,1771];
    let mut res = 0;

    for x in 0..nums.len() {
        if nums[x].to_string().len() % 2 == 0 {
            res += 1;
        }
    }
    println!("{:?}", res);
}


