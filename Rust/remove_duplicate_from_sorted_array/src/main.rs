// use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
       // [0,1,1,1,1,2,2,3,3,4]
        if nums.is_empty() {
            return 0;
        }
        let mut counter = 1;

        for i in 1..nums.len() {
            println!("Checking nums[{}] = {} and nums[{}] = {}", i, nums[i], counter-1, nums[counter-1] );
            if nums[i] != nums[counter-1] {
                nums[counter] = nums[i];
                counter+=1
            }
            println!("{:?}", nums)
        }

        counter as i32
    }
}

fn main() {
    let mut input = vec![1,1,2];
    let ans = 2;
    assert_eq!(ans, Solution::remove_duplicates(&mut input));
    println!("{:?}, {}", input.clone(), Solution::remove_duplicates(&mut input));

    let mut input = vec![0,0,1,1,1,2,2,3,3,4];
    let ans = 5;
    assert_eq!(ans, Solution::remove_duplicates(&mut input));
    println!("{:?}, {}", input.clone(), Solution::remove_duplicates(&mut input));

    let mut input = vec![1];
    let ans = 1;
    assert_eq!(ans, Solution::remove_duplicates(&mut input));
    println!("{:?}, {}", input.clone(), Solution::remove_duplicates(&mut input));

    let mut input = vec![1, 2, 3];
    let ans = 3;
    assert_eq!(ans, Solution::remove_duplicates(&mut input));
    println!("{:?}, {}", input.clone(), Solution::remove_duplicates(&mut input));

}
