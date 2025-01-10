/// # 2185. Counting Words With a Given Prefix
/// *Daily Challenge 2025-01-09*
/// You are given an array of strings ```words``` and a string ```pref```.
///
/// Return the number of strings in ```words``` that contain ```pref``` as a prefix.
///
/// A prefix of a string ```s``` is any leading contiguous substring of ```s```.
pub struct Solution;

impl Solution {
    /// First attempt accepted. Runtime of 0ms, this was so easy it was boring so I'm not even that proud.
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0i32;
        for word in words {
            if word.starts_with(&pref) {
                count += 1;
            }
        }
        count
    }
}

pub fn run() {
    assert_eq!(
        Solution::prefix_count(
            vec![
                "pay".to_string(),
                "attention".to_string(),
                "practice".to_string(),
                "attend".to_string()
            ],
            "at".to_string()
        ),
        2
    );
    assert_eq!(
        Solution::prefix_count(
            vec![
                "leetcode".to_string(),
                "win".to_string(),
                "loops".to_string(),
                "success".to_string()
            ],
            "code".to_string()
        ),
        0
    );
}
