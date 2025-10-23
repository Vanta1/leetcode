/// # 1488. Avoid Flood in the City
/// *Daily 2025-10-07*
/// TODO: .
pub struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let _full_lakes = HashSet::<i32>::new();
        let _solution = rains.iter().for_each(|day| if *day > 0 {});

        todo!()
    }
}

pub fn run() {
    assert_eq!(
        Solution::avoid_flood(vec![1, 2, 3, 4]),
        vec![-1, -1, -1, -1]
    );
    assert_eq!(
        Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
        vec![-1, -1, 2, 1, -1, -1]
    );
    assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
}
