/// # 1408. String Matching in an Array
/// *Daily Challenge 2025-01-07*
///
/// Given an array of string words, return all strings in words that is a substring of another word.
/// You can return the answer in any order.
///
/// A substring is a contiguous sequence of characters within a string
pub struct Solution;

impl Solution {
    /// My first attempt, brute-force solution. Accepted with a runtime of 2ms.
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut result: HashMap<String, bool> = HashMap::new();

        for (i, match_word) in words.iter().enumerate() {
            for (n, check_word) in words.iter().enumerate() {
                if i != n && match_word.contains(check_word) {
                    result.insert(check_word.to_string(), true);
                }
            }
        }

        result.into_keys().collect()
    }
}

pub fn run() {
    assert_eq!(
        {
            let mut s = Solution::string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string(),
            ]);
            s.sort();
            s
        },
        vec!["as".to_string(), "hero".to_string()]
    );
    assert_eq!(
        {
            let mut s =
                Solution::string_matching(vec!["leetcode".into(), "et".into(), "code".into()]);
            s.sort();
            s
        },
        vec!["code".to_string(), "et".to_string()]
    );
    assert_eq!(
        {
            let mut s = Solution::string_matching(vec!["blue".into(), "green".into(), "bu".into()]);
            s.sort();
            s
        },
        Vec::<String>::new()
    );
}
