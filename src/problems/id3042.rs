/// # 3042. Count Prefix and Suffix Pairs I
/// *Daily Challenge 2025-01-08*
///
/// You are given a 0-indexed string array ```words```.
///
/// Let's define a boolean function ```isPrefixAndSuffix``` that takes two strings, ```str1``` and ```str2```:
///
/// ```isPrefixAndSuffix(str1, str2)``` returns ```true``` if ```str1``` is both a prefix and a suffix of ```str2```, and false otherwise.
///
/// For example, ```isPrefixAndSuffix("aba", "ababa")``` is ```true``` because ```"aba"```
/// is a prefix of ```"ababa"``` and also a suffix, but ```isPrefixAndSuffix("abc", "abcd")``` is ```false```.
///
/// Return an integer denoting the number of index pairs ```(i, j)``` such that ```i < j```,
/// and ```isPrefixAndSuffix(words[i], words[j])``` is true.
pub struct Solution;

impl Solution {
    /// First attempt & solution, passed all tests with runtime of 1ms (only beat 40%).
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0i32;
        for (i, ps) in words.iter().enumerate() {
            for (j, word) in words.iter().enumerate() {
                if i >= j {
                    continue;
                }

                if word.starts_with(ps) && word.ends_with(ps) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn run() {
    assert_eq!(
        Solution::count_prefix_suffix_pairs(vec![
            "a".into(),
            "aba".into(),
            "ababa".into(),
            "aa".into(),
        ]),
        4
    );
    assert_eq!(
        Solution::count_prefix_suffix_pairs(vec![
            "pa".into(),
            "papa".into(),
            "ma".into(),
            "mama".into(),
        ]),
        2
    );
    assert_eq!(
        Solution::count_prefix_suffix_pairs(vec!["abab".into(), "ab".into(),]),
        0
    );
}
