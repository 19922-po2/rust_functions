use std::collections::{HashMap, HashSet};
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

/*
    Given a wordlist, we want to implement a spellchecker that converts a query word into a correct word.

    For a given query word, the spell checker handles two categories of spelling mistakes:

    Capitalization: If the query matches a word in the wordlist (case-insensitive), then the query word is returned with the same case as the case in the wordlist.
    Example: wordlist = ["yellow"], query = "YellOw": correct = "yellow"
    Example: wordlist = ["Yellow"], query = "yellow": correct = "Yellow"
    Example: wordlist = ["yellow"], query = "yellow": correct = "yellow"
    Vowel Errors: If after replacing the vowels ('a', 'e', 'i', 'o', 'u') of the query word with any vowel individually, it matches a word in the wordlist (case-insensitive), then the query word is returned with the same case as the match in the wordlist.
    Example: wordlist = ["YellOw"], query = "yollow": correct = "YellOw"
    Example: wordlist = ["YellOw"], query = "yeellow": correct = "" (no match)
    Example: wordlist = ["YellOw"], query = "yllw": correct = "" (no match)
    In addition, the spell checker operates under the following precedence rules:

    When the query exactly matches a word in the wordlist (case-sensitive), you should return the same word back.
    When the query matches a word up to capitlization, you should return the first such match in the wordlist.
    When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
    If the query has no matches in the wordlist, you should return the empty string.
    Given some queries, return a list of words answer, where answer[i] is the correct word for query = queries[i].

    Input: wordlist = ["KiTe","kite","hare","Hare"]
            queries = ["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"]
    Output:           ["kite","KiTe","KiTe","Hare","hare",""    ,""    ,"KiTe",""    ,"KiTe"]
*/
#[allow(unused)]
pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for querie in queries {
        if wordlist.contains(&querie) {
            result.push(querie);
        } else if check_capitalisation(&querie, &wordlist) {
            result.push(get_wordlist_word(&querie, &wordlist));
        } else if check_vowels_errors(&querie, &wordlist) {
            result.push(get_wordlist_word1(&querie, &wordlist));
        } else {
            result.push("".to_string());
        }
    }

    fn check_capitalisation(word: &String, wordlist: &Vec<String>) -> bool {
        let wordlist_lower: Vec<String> = wordlist.iter().map(|w| w.to_lowercase()).collect();
        if wordlist_lower.contains(&word.to_lowercase()) {
            return true;
        }
        if wordlist_lower.contains(&word.to_uppercase()) {
            return true;
        }
        return false;
    }

    fn get_wordlist_word(word: &String, wordlist: &Vec<String>) -> String {
        for w in wordlist {
            if w.to_lowercase() == word.to_lowercase() {
                return w.to_string();
            }
        }
        return "".to_string();
    }

    fn check_vowels_errors(word: &String, wordlist: &Vec<String>) -> bool {
        for w in wordlist {
            if w.to_lowercase()
                .chars()
                .map(|c| if "aeiouAEIOU".contains(c) { '_' } else { c })
                .collect::<String>()
                == word
                    .to_lowercase()
                    .chars()
                    .map(|c| if "aeiouAEIOU".contains(c) { '_' } else { c })
                    .collect::<String>()
            {
                return true;
            }
        }
        return false;
    }

    fn get_wordlist_word1(word: &String, wordlist: &Vec<String>) -> String {
        for w in wordlist {
            if w.to_lowercase()
                .chars()
                .map(|c| if "aeiouAEIOU".contains(c) { '_' } else { c })
                .collect::<String>()
                == word
                    .to_lowercase()
                    .chars()
                    .map(|c| if "aeiouAEIOU".contains(c) { '_' } else { c })
                    .collect::<String>()
            {
                return w.to_string();
            }
        }
        return "".to_string();
    }

    return result;
}

#[allow(unused)]
pub fn spellchecker_optimized(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut exact = HashSet::new();
    let mut case_map = HashMap::new();
    let mut vowel_map = HashMap::new();

    fn devowel(s: &str) -> String {
        s.chars()
            .map(|c| match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => '_',
                other => other,
            })
            .collect()
    }

    for word in &wordlist {
        exact.insert(word.clone());

        let lower = word.to_ascii_lowercase();
        case_map.entry(lower.clone()).or_insert(word.clone());

        let dev = devowel(&lower);
        vowel_map.entry(dev).or_insert(word.clone());
    }

    queries
        .into_iter()
        .map(|q| {
            if exact.contains(&q) {
                q
            } else {
                let lower = q.to_ascii_lowercase();
                if let Some(ans) = case_map.get(&lower) {
                    ans.clone()
                } else {
                    let dev = devowel(&lower);
                    vowel_map.get(&dev).cloned().unwrap_or_default()
                }
            }
        })
        .collect()
}

/*
    Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken,
    return the number of words in text you can fully type using this keyboard.
*/
#[allow(unused)]
pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut result = 0;
    let words: Vec<&str> = text.split_whitespace().collect();
    let letter_set: HashSet<char> = broken_letters.chars().collect();

    for word in words {
        if !(word.chars().any(|c| letter_set.contains(&c))) {
            result += 1;
        }
    }

    return result;
}

/*
    You are given an array of integers nums. Perform the following steps:

    Find any two adjacent numbers in nums that are non-coprime.
    If no such numbers are found, stop the process.
    Otherwise, delete the two numbers and replace them with their LCM (Least Common Multiple).
    Repeat this process as long as you keep finding two adjacent non-coprime numbers.
    Return the final modified array. It can be shown that replacing adjacent non-coprime numbers in any arbitrary order will lead to the same result.

    The test cases are generated such that the values in the final array are less than or equal to 108.

    Two values x and y are non-coprime if GCD(x, y) > 1 where GCD(x, y) is the Greatest Common Divisor of x and y.
*/
#[allow(unused)]
pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for num in nums {
        let mut curr = num;
        while let Some(&last) = stack.last() {
            let g = gcd(curr, last);
            if g == 1 {
                break;
            }
            stack.pop();
            curr = lcm(curr, last);
        }
        stack.push(curr);
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        (a / gcd(a, b)) * b
    }

    stack
}

/*
    You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

    Merge nums1 and nums2 into a single array sorted in non-decreasing order.

    The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
    To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored.
    nums2 has a length of n.
*/
#[allow(unused)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for j in 0..nums2.len() {
        for i in j..nums1.len() {
            if nums2[j as usize] < nums1[i as usize] {
                nums1.remove(nums1.len() - 1);
                nums1.insert(i as usize, nums2[j as usize]);
                break;
            } else if i >= m as usize + j {
                // else if nums1[i as usize] == 0
                nums1.remove(i as usize);
                nums1.insert(i as usize, nums2[j as usize]);
                break;
            }
        }
        println!("{:?}", nums1);
    }
    println!("nums1: {:?}", nums1);
}

/*
    You are given a string s consisting of lowercase English letters ('a' to 'z').

    Your task is to:

    Find the vowel (one of 'a', 'e', 'i', 'o', or 'u') with the maximum frequency.
    Find the consonant (all other letters excluding vowels) with the maximum frequency.
    Return the sum of the two frequencies.

    Note: If multiple vowels or consonants have the same maximum frequency, you may choose any one of them.
    If there are no vowels or no consonants in the string, consider their frequency as 0.

    The frequency of a letter x is the number of times it occurs in the string.
*/
#[allow(unused)]
pub fn max_freq_sum(s: String) -> i32 {
    let mut map = HashMap::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    let max_vowel_count = map
        .iter()
        .filter(|(k, _)| vowels.contains(k))
        .map(|(_, &v)| v)
        .max()
        .unwrap_or(0);

    let max_const_count = map
        .iter()
        .filter(|(k, _)| !vowels.contains(k))
        .map(|(_, &v)| v)
        .max()
        .unwrap_or(0);

    println!("{:?}", map);

    return max_vowel_count + max_const_count;
}

/*
    Given two binary strings a and b, return their sum as a binary string.
*/
#[allow(unused)]
pub fn add_binary(a: String, b: String) -> String {
    let mut result = "".to_string();

    let mut rev_a: Vec<char> = a
        .chars() /* .rev() */
        .collect();
    let mut rev_b: Vec<char> = b
        .chars() /* .rev() */
        .collect();
    pad_start_with_zero(&mut rev_a, &mut rev_b);
    let mut rev_a: Vec<char> = rev_a.into_iter().rev().collect();
    let mut rev_b: Vec<char> = rev_b.into_iter().rev().collect();

    let max = rev_a.len().max(rev_b.len());

    let mut temp: i32 = 0;
    for i in 0..max {
        let ch: char = std::char::from_digit(
            add(
                rev_a[i].to_digit(10).unwrap(),
                rev_b[i].to_digit(10).unwrap(),
                &mut temp,
            ),
            10,
        )
        .unwrap_or('0');
        result.push(ch);
    }
    if temp == 1 {
        result.push('1');
    }

    fn add(a: u32, b: u32, temp: &mut i32) -> u32 {
        if *temp == 0 {
            if a + b == 2 {
                *temp = 1;
                return 0;
            } else if a + b == 1 {
                return 1;
            } else {
                return 0;
            }
        } else if *temp == 1 {
            if a + b == 2 {
                return 1;
            } else if a + b == 1 {
                return 0;
            } else {
                *temp = 0;
                return 1;
            }
        } else {
            println!("else");
            return 0;
        }
    }

    fn pad_start_with_zero(a: &mut Vec<char>, b: &mut Vec<char>) {
        let max_len = a.len().max(b.len());

        while a.len() < max_len {
            a.insert(0, '0');
        }

        while b.len() < max_len {
            b.insert(0, '0');
        }
    }

    return result.chars().rev().collect();
}

/*
    Given two strings s and t, determine if they are isomorphic.
    Two strings s and t are isomorphic if the characters in s can be replaced to get t.
    All occurrences of a character must be replaced with another character while preserving the order of characters.
    No two characters may map to the same character, but a character may map to itself.
*/
#[allow(unused)]
pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (cs, ct) in s.chars().zip(t.chars()) {
        // Check s -> t mapping
        if let Some(&mapped) = s_to_t.get(&cs) {
            if mapped != ct {
                return false;
            }
        } else {
            s_to_t.insert(cs, ct);
        }

        // Check t -> s mapping
        if let Some(&mapped) = t_to_s.get(&ct) {
            if mapped != cs {
                return false;
            }
        } else {
            t_to_s.insert(ct, cs);
        }
    }

    true
}

/*
    Given an integer array nums, return the number of subarrays of length 3 such that the sum of the first and third numbers equals exactly half of the second number.
*/
#[allow(unused)]
pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left_pointer = 0;
    let mut right_pointer = 2;

    while right_pointer < nums.len() {
        if check_condition(nums[left_pointer], nums[right_pointer - 1], nums[right_pointer]) {
            result += 1;
        }
        left_pointer += 1;
        right_pointer += 1;
    }

    return result;

    fn check_condition(first: i32, second: i32, third: i32) -> bool {
        return (first + third) as f32 == second as f32 / 2.0;
    }
}

/*
    You are given an array nums consisting of positive integers.
    Return the total frequencies of elements in nums such that those elements all have the maximum frequency.
    The frequency of an element is the number of occurrences of that element in the array.
*/
#[allow(unused)]
pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut map = HashMap::new();

    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }
    if let Some(&max_val) = map.values().max() {
        result = map.values().filter(|&&v| v == max_val).sum();
    }

    return result;
}

/*
    Given two version strings, version1 and version2, compare them. A version string consists of revisions separated by dots '.'.
    The value of the revision is its integer conversion ignoring leading zeros.

    To compare version strings, compare their revision values in left-to-right order. If one of the version strings has fewer revisions, treat the missing revision values as 0.

    Return the following:

    If version1 < version2, return -1.
    If version1 > version2, return 1.
    Otherwise, return 0.
*/
#[allow(unused)]
pub fn compare_version(version1: String, version2: String) -> i32 {
    let version1_nums: Vec<i32> = version1.split('.').map(|s| s.parse::<i32>().unwrap()).collect();

    let version2_nums: Vec<i32> = version2.split('.').map(|s| s.parse::<i32>().unwrap()).collect();

    let max_len = version1_nums.len().max(version2_nums.len());

    for i in 0..max_len {
        let v1 = *version1_nums.get(i).unwrap_or(&0);
        let v2 = *version2_nums.get(i).unwrap_or(&0);

        if v1 > v2 {
            return 1;
        } else if v1 < v2 {
            return -1;
        }
    }
    return 0;
}

/*
    Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.

    If the fractional part is repeating, enclose the repeating part in parentheses.

    If multiple answers are possible, return any of them.

    It is guaranteed that the length of the answer string is less than 104 for all the given inputs.
*/
#[allow(unused)]
pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string();
    }

    let mut result = String::new();

    // Handle sign
    let negative = (numerator < 0) ^ (denominator < 0);
    let (num, den) = ((numerator as i64).abs(), (denominator as i64).abs());
    if negative {
        result.push('-');
    }

    // Integer part
    result.push_str(&(num / den).to_string());

    let mut remainder = num % den;
    if remainder == 0 {
        return result;
    }

    result.push('.');

    // Fractional part
    let mut seen: HashMap<i64, usize> = HashMap::new();

    while remainder != 0 {
        if let Some(&pos) = seen.get(&remainder) {
            result.insert(pos, '(');
            result.push(')');
            break;
        }

        seen.insert(remainder, result.len());

        remainder *= 10;
        result.push_str(&(remainder / den).to_string());
        remainder %= den;
    }

    result
}

/*
    You are given an integer array nums and a positive integer k.

    Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.

    A subarray is a contiguous sequence of elements within an array.
*/
#[allow(unused)]
pub fn count_subarrays_1(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut ans: i64 = 0;

    // Find the global maximum
    let global_max = *nums.iter().max().unwrap();

    // Collect indices of global_max
    let mut idx = vec![];
    for (i, &x) in nums.iter().enumerate() {
        if x == global_max {
            idx.push(i as i64);
        }
    }

    if (idx.len() as i32) < k {
        return 0; // not enough occurrences at all
    }

    // For each k-window of max indices
    for j in 0..=idx.len() - (k as usize) {
        let left_idx = idx[j];
        let right_idx = idx[j + (k as usize) - 1];

        // left choices
        let prev = if j > 0 { idx[j - 1] } else { -1 };
        let left_choices = left_idx - prev;

        // right choices: can extend until the end of array
        let right_choices = (n as i64) - right_idx;

        ans += left_choices * right_choices;
    }

    ans
}

/*
    Given an integer array nums, return the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.
*/
#[allow(unused)]
pub fn triangle_number(nums: Vec<i32>) -> i32 {
    let mut mut_nums = nums;
    mut_nums.sort(); // sort the array
    let n = mut_nums.len();
    let mut count = 0;

    for k in (2..n).rev() {
        // fix the largest side
        let mut i = 0;
        let mut j = k - 1;

        while i < j {
            if mut_nums[i] + mut_nums[j] > mut_nums[k] {
                // all pairs (i..j-1, j) are valid
                count += (j - i) as i32;
                j -= 1;
            } else {
                i += 1;
            }
        }
    }

    count
}

/*
    Given an array of points on the X-Y plane points where points[i] = [xi, yi],
    return the area of the largest triangle that can be formed by any three different points.
    Answers within 10-5 of the actual answer will be accepted.
*/
#[allow(unused)]
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let n = points.len();
    let mut max_area = 0.0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let (x1, y1) = (points[i][0] as f64, points[i][1] as f64);
                let (x2, y2) = (points[j][0] as f64, points[j][1] as f64);
                let (x3, y3) = (points[k][0] as f64, points[k][1] as f64);

                let area = ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs()) / 2.0;

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

/*
    Write a function to find the longest common prefix string amongst an array of strings.

    If there is no common prefix, return an empty string "".
*/
#[allow(unused)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix.pop(); // remove last character
        }
    }

    prefix
}

/*
    Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.
    The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

    Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially.
    The remaining elements of nums are not important as well as the size of nums.
    Return k.
*/
#[allow(unused)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i = 0;

    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }

    (i + 1) as i32
}
