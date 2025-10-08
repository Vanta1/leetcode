/// # 407. Trapping Rain Water II
/// *Daily 2025-10-03*
/// TODO: .
pub struct Solution;

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "z: {}, t: {}", self.z, self.t)
    }
}
impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &CellType::Block => String::from("Block"),
                &CellType::Candidate(b) => format!("{b:04b}"),
            }
        )
    }
}

struct Cell {
    z: u32,
    t: CellType,
}

#[derive(Debug)]
enum CellType {
    Block,
    Candidate(u8),
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut hm: Vec<Vec<Cell>> = height_map
            .iter()
            .map(|v| {
                v.iter()
                    .map(|c| Cell {
                        z: (*c as u32),
                        t: CellType::Block,
                    })
                    .collect()
            })
            .collect();
        let mut total = 0;
        let mut changed = true;

        while changed {
            changed = false;
            for y in 1..(hm.len() - 1) {
                for x in 1..(hm[0].len() - 1) {
                    // TODO: match against hm[y][x]'s type, then if it's a candidate check bordering candidates

                    let b = (
                        ((hm[y][x].z < hm[y - 1][x].z) as u8) << 3) // N
                        + (((hm[y][x].z < hm[y][x + 1].z) as u8) << 2) // E
                        + (((hm[y][x].z < hm[y + 1][x].z) as u8) << 1) // S
                        + ((hm[y][x].z < hm[y][x - 1].z) as u8 // W
                    );

                    if b == 0b1111 {
                        total += 1;
                        hm[y][x].z += 1;
                        changed = true;
                    } else if b >= 1 {
                        hm[y][x].t = CellType::Candidate(b);
                    }
                }
            }
        }

        dbg!(hm);

        total
    }
}

pub fn run() {
    //assert_eq!(
    //    Solution::trap_rain_water(vec![
    //        vec![1, 4, 3, 1, 3, 2],
    //        vec![3, 2, 1, 3, 2, 4],
    //        vec![2, 3, 3, 2, 3, 1]
    //    ]),
    //    4
    //);
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
