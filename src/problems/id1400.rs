/// # 1400. Construct K Palindrome Strings
/// *Daily Challenge 2025-01-11*
///
/// Given a string ```s``` and an integer ```k```, return ```true``` if you can use all the characters
/// in ```s``` to construct ```k``` palindrome strings or ```false``` otherwise.
pub struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        todo!()
    }
}

pub fn run() {
    // didn't use assert_eq! because can_construct returns a bool already
    assert!(Solution::can_construct("annabelle".to_string(), 2));
    assert!(!Solution::can_construct("leetcode".to_string(), 3));
    assert!(Solution::can_construct("true".to_string(), 4));
}
