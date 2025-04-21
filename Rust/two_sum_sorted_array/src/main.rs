struct Solution;

impl Solution {
    // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
    // O(n) time complexity and O(n) space complexity
    // 1ms | 2.31MB

    pub fn two_sum_sorted_array(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // handle edge case
        if nums.is_empty() {
            return vec![]
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        while low < high {
            if (nums[low] + nums[high]) == target {
                return vec![(low + 1) as i32, (high + 1) as i32]
            } else if (nums[low] + nums[high]) > target {
                high -= 1
            } else {
                low += 1
            }
        }
        vec![]
    }
}

fn main() {
    let arr = vec![2,7,11,15];
    let tar = 9;
    let ans = vec![1,2];
    assert_eq!(ans, Solution::two_sum_sorted_array(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum_sorted_array(arr, tar));

    

    let arr = vec![3,3];
    let tar = 6;
    let ans = vec![1,2];
    assert_eq!(ans, Solution::two_sum_sorted_array(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum_sorted_array(arr, tar));

    let arr = vec![2,3,4];
    let tar = 6;
    let ans = vec![1,3];
    assert_eq!(ans, Solution::two_sum_sorted_array(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum_sorted_array(arr, tar));

    let arr = vec![];
    let tar = 1;
    let ans: Vec<i32> = vec![];
    assert_eq!(ans, Solution::two_sum_sorted_array(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum_sorted_array(arr, tar));
}
