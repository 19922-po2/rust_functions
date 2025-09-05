/*
    You are given two integers num1 and num2.
    In one operation, you can choose integer i in the range [0, 60] and subtract 2i + num2 from num1.
    Return the integer denoting the minimum number of operations needed to make num1 equal to 0.
    If it is impossible to make num1 equal to 0, return -1.
*/

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
