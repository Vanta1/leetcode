/// # 407. Trapping Rain Water II
/// *Daily 2025-10-03*
pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        let mut hm = height_map;

        let mut total = 0;

        for y in 1..(hm.len() - 1) {
            for x in 1..(hm[0].len() - 1) {
                if hm[y][x] < hm[y][x - 1]
                    && hm[y][x] < hm[y - 1][x]
                    && hm[y][x] < hm[y][x + 1]
                    && hm[y][x] < hm[y + 1][x]
                {
                    total += min(
                        min(hm[y][x - 1], hm[y - 1][x]),
                        min(hm[y][x + 1], hm[y + 1][x]),
                    ) - hm[y][x];
                }
            }
        }

        total
    }
}

pub fn run() {
    assert_eq!(
        Solution::trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]),
        4
    );
    assert_eq!(
        Solution::trap_rain_water(vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3]
        ]),
        10
    );
}
