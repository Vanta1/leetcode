/// # 916. Word Subsets
/// *Daily Challenge 2025-01-10*
///
/// You are given two string arrays ```words1``` and ```words2```.
///
/// A string ```b``` is a subset of string ```a``` if every letter in ```b``` occurs in ```a``` including multiplicity.
///
/// For example, ```"wrr"``` is a subset of ```"warrior"``` but is not a subset of ```"world"```.
///
/// A string ```a``` from ```words1``` is universal if for every string ```b``` in ```words2```, ```b``` is a subset of ```a```.
///
/// Return an array of all the universal strings in ```words1```. You may return the answer in any order.
pub struct Solution;

impl Solution {
    /// This was my first solution, and I got a runtime of 10ms, which as of writing beats 75% of submissions.
    /// So I'm happy and will stick with this, sorry if variables are poorly named I only spent like 20 mins on this.
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut b_count = [0u8; 26];
        for b in words2 {
            let mut letters = [0u8; 26];
            for c in b.chars() {
                letters[((c as u8) - 97) as usize] += 1;
            }
            for (i, letter) in letters.iter().enumerate() {
                b_count[i] = std::cmp::max(b_count[i], *letter);
            }
        }
        let mut result: Vec<String> = Vec::new();
        'out: for a in words1 {
            let mut letters = [0u8; 26];
            for c in a.chars() {
                letters[((c as u8) - 97) as usize] += 1;
            }
            for (i, letter) in letters.iter().enumerate() {
                if b_count[i] > *letter {
                    continue 'out;
                }
            }
            result.push(a.clone());
        }
        result
    }
}

pub fn run() {
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["e".to_string(), "o".to_string()]
        ),
        vec![
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string()
        ]
    );
    assert_eq!(
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ],
            vec!["e".to_string(), "l".to_string()]
        ),
        vec![
            "apple".to_string(),
            "google".to_string(),
            "leetcode".to_string()
        ]
    );
}
