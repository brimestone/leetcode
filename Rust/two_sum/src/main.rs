struct Solution;

impl Solution {
    // https://leetcode.com/problems/two-sum/
    // O(n) time complexity and O(n) space complexity
    // 19ms | 2.35MB

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx, valx) in nums.clone().into_iter().enumerate() {
            let complement = target - valx;
            
            for (idy, valy) in nums[idx+1..].into_iter().enumerate() {
                if *valy == complement {
                    return vec![idx as i32, (idy + idx+1) as i32]
                }
            }
        }
        vec![0]
    }
}

fn main() {
    let arr = vec![2,7,11,15];
    let tar = 9;
    let ans = vec![0,1];
    assert_eq!(ans, Solution::two_sum(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum(arr, tar));

    let arr = vec![3,2,4];
    let tar = 6;
    let ans = vec![1,2];
    
    assert_eq!(ans, Solution::two_sum(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum(arr, tar));

    let arr = vec![3,3];
    let tar = 6;
    let ans = vec![0,1];
    assert_eq!(ans, Solution::two_sum(arr.clone(), tar.clone()));
    println!("Two sum: {:?}", Solution::two_sum(arr, tar));
}
