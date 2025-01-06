pub struct Solution;

impl Solution {
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
        unreachable!()
    }
}

pub fn run() {
    dbg!(Solution::two_sum(vec![2, 7, 11, 15], 9));
    dbg!(Solution::two_sum(vec![3, 2, 4], 6));
    dbg!(Solution::two_sum(vec![3, 3], 6));
}
