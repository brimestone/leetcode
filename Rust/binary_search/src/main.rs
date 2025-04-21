
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1
        }
        
        let mut left = 0;
        let mut right = nums.len();

        -1
    }
}

fn main() {
    let input = vec![];
    let target = 9;
    let ans = -1;
    assert_eq!(ans, Solution::search(input.clone(), target));
    println!("Checked - {:?}, {}", input.clone(), Solution::search(input, target));

    
    let input = vec![-1,0,3,5,9,12];
    let target = 9;
    let ans = 4;
    assert_eq!(ans, Solution::search(input.clone(), target));
    println!("{:?}, {}", input.clone(), Solution::search(input, target));


}
