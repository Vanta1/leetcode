/// # 1408. String Matching in an Array
/// *Daily Challenge 2025-01-07*
///
/// Given an array of string words, return all strings in words that is a substring of another word.
/// You can return the answer in any order.
///
/// A substring is a contiguous sequence of characters within a string
pub struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        vec![]
    }
}

pub fn run() {
    assert_eq!(
        Solution::string_matching(vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string(),
        ]),
        vec!["as".to_string(), "hero".to_string()]
    );
    assert_eq!(
        Solution::string_matching(vec!["leetcode".into(), "et".into(), "code".into(),]),
        vec!["et".to_string(), "code".to_string()]
    );
    assert_eq!(
        Solution::string_matching(vec!["blue".into(), "green".into(), "bu".into(),]),
        Vec::<String>::new()
    );
}
