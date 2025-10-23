use std::u8;

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
                &CellType::StrongCandidate(b) => format!("S|{b:04b}"),
            }
        )
    }
}

struct Cell {
    z: u32,
    t: CellType,
}

#[derive(Debug, PartialEq)]
enum CellType {
    Block,
    Candidate(u8),
    StrongCandidate(u8),
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut hm: Vec<Vec<Cell>> = height_map
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|c| Cell {
                        z: (c as u32),
                        t: CellType::Block,
                    })
                    .collect()
            })
            .collect();
        let mut total = 0;
        let mut changed = true;

        // so far, this only fills all entirely surrounded cells
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
                        match hm[y][x].t {
                            CellType::Block => {
                                hm[y][x].t = CellType::Candidate(b);
                                changed = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // after, we have *candidates* but not all of them

        changed = true;
        while changed {
            changed = false;
            for y in 1..(hm.len() - 1) {
                for x in 1..(hm[0].len() - 1) {
                    match hm[y][x].t {
                        CellType::Candidate(b) => {
                            let b = !b; // flip all bits in b
                            let mut c_b = 0;

                            // check if b @ index 0..4 (0-3, inclusive) is a Candidate, if they all are, this becomes a StrongCandidate
                            for n in 0..4 {
                                if b & u8::pow(2, n) == u8::pow(2, n) {
                                    match hm[y - 1][x].t {
                                        CellType::Candidate(_) => c_b += 1,
                                        _ => {}
                                    }
                                }
                            }

                            if c_b == b {
                                hm[y][x].t = CellType::StrongCandidate(!b) // remember to un-flip b
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        dbg!(hm);

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
