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
