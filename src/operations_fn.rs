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


