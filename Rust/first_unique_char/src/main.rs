use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // https://leetcode.com/problems/first-unique-character-in-a-string/
        // O(n) time complexity and O(n) space complexity
        // 8ms | 2.36MB
        let mut seen:HashMap<char, i32> = HashMap::with_capacity(26);

        for (i, c) in s.chars().enumerate() {
            seen.entry(c).and_modify(|count| *count = -1).or_insert(i as i32);
        }

        for (i, c) in s.chars().enumerate() {
            if let Some(&count) = seen.get(&c) {
                if count != -1 as i32 {
                    return i as i32;
                }
            }
        }
        -1
    }
}

fn main() {
    let input = "leetcode";
    let ans = 0;
    assert_eq!(ans, Solution::first_uniq_char(input.to_string()));
    println!("First unique char happens in index: {}", Solution::first_uniq_char(input.to_string()));

    let input = "loveleetcode";
    let ans = 2;
    assert_eq!(ans, Solution::first_uniq_char(input.to_string()));
    println!("First unique char happens in index: {}", Solution::first_uniq_char(input.to_string()));

    let input = "aabb";
    let ans = -1;
    assert_eq!(ans, Solution::first_uniq_char(input.to_string()));
    println!("First unique char happens in index: {}", Solution::first_uniq_char(input.to_string()));

    let input = "";
    let ans = -1;
    assert_eq!(ans, Solution::first_uniq_char(input.to_string()));
    println!("First unique char happens in index: {}", Solution::first_uniq_char(input.to_string()));
}
