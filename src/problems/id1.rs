/// # 1. Two Sum
///
/// Given an array of integers ```nums``` and an integer ```target```,
/// return indices of the two numbers such that they add up to ```target```.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
pub struct Solution;

impl Solution {
    /// Solution from the [editorial](https://leetcode.com/problems/two-sum/editorial/). runtime of 0ms
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            match map.get(&num) {
                Some(&j) => return vec![i as i32, j as i32],
                None => {
                    map.insert(target - num, i);
                }
            }
        }
        vec![]
    }

    // my first solution, very slow runtime of 62ms but passed all tests
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut solution: Vec<i32> = vec![0, 0];
    //     for i in 0..nums.len() {
    //         for n in 0..nums.len() {
    //             if i != n && (nums[i] + nums[n]) as i32 == target {
    //                 solution = vec![i as i32, n as i32];
    //             }
    //         }
    //     }
    //     return solution;
    // }
}

pub fn run() {
    assert_eq!(
        {
            let mut s = Solution::two_sum(vec![2, 7, 11, 15], 9);
            s.sort();
            s
        },
        vec![0, 1]
    );
    assert_eq!(
        {
            let mut s = Solution::two_sum(vec![3, 2, 4], 6);
            s.sort();
            s
        },
        vec![1, 2]
    );
    assert_eq!(
        {
            let mut s = Solution::two_sum(vec![3, 3], 6);
            s.sort();
            s
        },
        vec![0, 1]
    );
}
