struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // https://leetcode.com/problems/palindrome-number
        // O(n) time complexity and O(n) space complexity
        // 4ms | 2.37MB

        let x_as_string = format!("{}", x);
        let x_as_vec = x_as_string.chars().collect::<Vec<_>>();
        let mut counter_x = 0;
        let mut counter_y = x_as_string.len() - 1;

        if x_as_string.len() == 2 && x_as_vec[0] != x_as_vec[1] {
            return false
        }

        while counter_x < ((x_as_string.len()) / 2 ) {
            if x_as_vec[counter_x] != x_as_vec[counter_y] {
                return false
            }
            counter_x +=1 ;
            counter_y -=1 ;
        }
        true
    }
}

fn main() {
    let input = 121;
    let ans = true;
    println!("Is number palindrome: {}", Solution::is_palindrome(input));
    assert_eq!(ans, Solution::is_palindrome(input));

    let input = -121;
    let ans = false;
    println!("Is number palindrome: {}", Solution::is_palindrome(input));
    assert_eq!(ans, Solution::is_palindrome(input));

    let input = 10;
    let ans = false;
    println!("Is number palindrome: {}", Solution::is_palindrome(input));
    assert_eq!(ans, Solution::is_palindrome(input));

    let input = 11;
    let ans = true;
    println!("Is number palindrome: {}", Solution::is_palindrome(input));
    assert_eq!(ans, Solution::is_palindrome(input));

    let input = 1000030001;
    let ans = false;
    println!("Is number palindrome: {}", Solution::is_palindrome(input));
    assert_eq!(ans, Solution::is_palindrome(input));
    
    
}
