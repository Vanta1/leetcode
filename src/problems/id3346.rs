use std::i32;

/// # 3346. Maximum Frequency of an Element After Performing Operations I
/// *Daily 2025-10-21*
pub struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::cmp::{max, min};

        let mut eq_pairs = vec![0i32; nums.len()];
        let mut in_range = vec![0i32; nums.len()];

        for i in 0..nums.len() {
            let mut ops_left = num_operations;
            for n in 0..nums.len() {
                if nums[i] == nums[n] {
                    eq_pairs[i] += 1;
                } else if ops_left >= 1 && nums[i] - k <= nums[n] && nums[i] + k >= nums[n] {
                    in_range[i] += 1;
                    ops_left -= 1;
                } else if ops_left >= 2
                    && max(nums[i] - k, nums[n] - k) < min(nums[i] + k, nums[n] + k)
                {
                    in_range[i] += 1;
                    ops_left -= 2;
                }
            }
        }

        let mut max_freq = 0;

        eq_pairs
            .iter()
            .zip(in_range.iter())
            .map(|(i, n)| i + min(*n, num_operations))
            .for_each(|n| max_freq = max(max_freq, n));

        max_freq
    }
}

pub fn run() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
    assert_eq!(Solution::max_frequency(vec![4, 11, 20, 20], 5, 1), 2);
    assert_eq!(Solution::max_frequency(vec![2, 70, 73], 39, 2), 2);
    assert_eq!(Solution::max_frequency(vec![23, 54], 77, 1), 2);
}
