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
