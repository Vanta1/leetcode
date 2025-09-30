/// # 2221. Find Triangular Sum of an Array
/// *Daily 2025-09-30*
///
/// You are given a 0-indexed integer array `nums`, where `nums[i]` is a digit between `0` and `9` (inclusive).
///
/// The triangular sum of `nums` is the value of the only element present in `nums` after the following process terminates:
///
/// Let `nums` comprise of `n` elements. If `n == 1`, end the process. Otherwise, create a new 0-indexed integer array `newNums` of length `n - 1`.
/// For each index `i`, where `0 <= i < n - 1`, assign the value of `newNums[i]` as `(nums[i] + nums[i+1]) % 10`, where % denotes modulo operator.
/// Replace the array `nums` with `newNums`.
/// Repeat the entire process starting from step 1.
///
/// Return the triangular sum of `nums`.
pub struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while nums.len() != 1 {
            let mut n = vec![0; nums.len() - 1];
            for i in 0..n.len() {
                n[i] = (nums[i] + nums[i + 1]) % 10;
            }
            nums = n;
        }

        nums[0]
    }
}

pub fn run() {
    assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    assert_eq!(Solution::triangular_sum(vec![5]), 5);
}
