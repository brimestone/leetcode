struct Solution;

impl Solution {
    // https://leetcode.com/problems/roman-to-integer
    // O(n) time complexity and O(n) space complexity
    // 2ms | 2.13MB
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let mut total: i32 = 0;
        let mut converted_arr: Vec<i32> = Vec::with_capacity(s.len());

        for roman in s.chars() {
            converted_arr.push(value(roman));
        }

        let mut counter = 0;
        let len_of_arr = converted_arr.len() - 1;

        while counter < len_of_arr {
            if converted_arr[counter] < converted_arr[counter + 1] {
                // If next is bigger than current, remove it from total
                total -= converted_arr[counter];
                counter+=1;
            } else {
                total += converted_arr[counter];
                counter+=1;
            }
        }
        
        if counter == len_of_arr {
            total += converted_arr[counter];
        }

        println!("Converting  `{}` to int: {}", s, total.clone());
        total
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IIII".to_string()), 4); // Shoudn't work! Invalid format
    assert_eq!(Solution::roman_to_int("VV".to_string()), 10); // Shoudn't work! Invalid format
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(Solution::roman_to_int("I".to_string()), 1);
    assert_eq!(Solution::roman_to_int("II".to_string()), 2);
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("V".to_string()), 5);
    assert_eq!(Solution::roman_to_int("VI".to_string()), 6);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("X".to_string()), 10);
    assert_eq!(Solution::roman_to_int("XIII".to_string()), 13);
    assert_eq!(Solution::roman_to_int("XXVII".to_string()), 27);
    assert_eq!(Solution::roman_to_int("XL".to_string()), 40);
    assert_eq!(Solution::roman_to_int("L".to_string()), 50);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("XC".to_string()), 90);
    assert_eq!(Solution::roman_to_int("C".to_string()), 100);
    assert_eq!(Solution::roman_to_int("CXLIV".to_string()), 144);
    assert_eq!(Solution::roman_to_int("CD".to_string()), 400);
    assert_eq!(Solution::roman_to_int("D".to_string()), 500);
    assert_eq!(Solution::roman_to_int("CM".to_string()), 900);
    assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(Solution::roman_to_int("MMM".to_string()), 3000);
    assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);
}
