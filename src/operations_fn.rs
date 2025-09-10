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
    let nums = [555, 901, 482, 1771];
    let mut res = 0;

    for x in 0..nums.len() {
        if nums[x].to_string().len() % 2 == 0 {
            res += 1;
        }
    }
    println!("{:?}", res);
}

/*
    Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d),
    or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.
    Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].
*/
#[allow(unused)]
pub fn num_equiv_domino_pairs1() {
    // brute force
    let dominoes = [[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]];
    let mut res = 0;

    for x in 0..dominoes.len() {
        for y in x + 1..dominoes.len() {
            if check_equality(dominoes[x], dominoes[y]) {
                res += 1;
            }
        }
    }

    fn check_equality(i: [i32; 2], j: [i32; 2]) -> bool {
        return i[0] == j[0] && i[1] == j[1] || i[0] == j[1] && i[1] == j[0];
    }

    println!("{:?}", res);
}

#[allow(unused)]
pub fn num_equiv_domino_pairs2() {
    //map
    //let dominoes = [[1, 1], [2, 2], [1, 1], [1, 2], [1, 2], [1, 1]]; // output: 4
    //let dominoes = [[1,2],[2,1],[3,4],[5,6]]; // output: 1
    let dominoes = [[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]; // output: 3
    //let dominoes = [[2,1],[1,2],[1,2],[1,2],[2,1],[1,1],[1,2],[2,2]];
    let mut res = 0;
    let mut map = HashMap::new();

    for x in 0..dominoes.len() {
        *map.entry(if &dominoes[x][0] >= &dominoes[x][1] {
            [&dominoes[x][0], &dominoes[x][1]]
        } else {
            [&dominoes[x][1], &dominoes[x][0]]
        })
        .or_insert(0) += 1;
    }

    fn calc_pairs(elements: &i32) -> i32 {
        if elements > &2 {
            return (elements * (elements - 1)) / 2;
        } else {
            return 1;
        };
    }

    let values: Vec<&i32> = map.values().filter(|&&v| v > 1).collect();
    for v in 0..values.len() {
        if values[v] > &1 {
            res += calc_pairs(&values[v]);
        }
    }
    println!("res: {:?}", res);
}

/*
    Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.
*/
#[allow(unused)]
pub fn three_consecutive_odds() {
    let arr = [1, 2, 34, 3, 4, 5, 7, 23, 12];
    let mut count = 0;

    for x in 0..arr.len() {
        if count >= 3 {
            break;
        }
        if arr[x] % 2 != 0 {
            count += 1
        } else {
            count = 0;
        }
    }

    println!("res: {:?}", if count >= 3 { true } else { false });
}

/*
    You are given an integer array digits, where each element is a digit. The array may contain duplicates.

    You need to find all the unique integers that follow the given requirements:

    The integer consists of the concatenation of three elements from digits in any arbitrary order.
    The integer does not have leading zeros.
    The integer is even.
    For example, if the given digits were [1, 2, 3], integers 132 and 312 follow the requirements.

    Return a sorted array of the unique integers.
*/
#[allow(unused)]
pub fn find_even_numbers() {
    let digits = [2, 1, 3, 0];
    // results are within 100~999 - only evens
    let mut res: Vec<i32> = Vec::new();

    for x in (100..999).step_by(2) {
        let digits_vec: Vec<i32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let digits_array: [i32; 3] = digits_vec.try_into().expect("Wrong number of digits");

        //println!("comparing: {:?} and {:?}", &digits_array, &digits);
        if contains_all_with_counts(&digits, &digits_array) {
            //println!("suscess: {:?}", x);
            res.push(x);
        }
    }

    fn contains_all_with_counts(first: &[i32], second: &[i32]) -> bool {
        let mut counts = HashMap::new();

        // Count occurrences in the first array
        for &num in first {
            *counts.entry(num).or_insert(0) += 1;
        }
        println!("occurrences: {:?}", counts);

        // Check if the second array can be formed from the first
        for &num in second {
            let entry = counts.entry(num).or_insert(0);
            if *entry == 0 {
                return false; // not enough of this number
            }
            *entry -= 1; // use one occurrence
        }
        true
    }

    println!("res: {:?}", res);
}
