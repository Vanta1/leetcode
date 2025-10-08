/// # 1518. Water Bottles
/// *Daily Challenge 2025-10-01
/// TODO: .
pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut full = num_bottles;
        let mut empty = 0;
        let mut total = num_bottles;
        while full > num_exchange {
            total += full;
            empty += full;
            full = empty.div_euclid(num_exchange);
            empty = empty % num_exchange;
        }

        total
    }
}

pub fn run() {
    assert_eq!(Solution::num_water_bottles(9, 3), 13);
    assert_eq!(Solution::num_water_bottles(15, 4), 19);
}
