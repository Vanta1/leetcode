/// # 1400. Construct K Palindrome Strings
/// *Daily Challenge 2025-01-11*
///
/// Given a string ```s``` and an integer ```k```, return ```true``` if you can use all the characters
/// in ```s``` to construct ```k``` palindrome strings or ```false``` otherwise.
pub struct Solution;

impl Solution {
    /// Used hints to help solve, first submission due to issue with test case 93, or the 4th test in run().
    /// Second submission passed with a runtime of 0ms.
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let mut counts = [0u32; 26];
        for c in s.chars() {
            counts[((c as u8) - 97) as usize] += 1;
        }
        let odds: Vec<u32> = counts.into_iter().filter(|n| n % 2 != 0).collect();
        if odds.len() > k as usize {
            return false;
        }
        true
    }
}

pub fn run() {
    // didn't use assert_eq! because can_construct returns a bool already
    assert!(Solution::can_construct("annabelle".to_string(), 2));
    assert!(!Solution::can_construct("leetcode".to_string(), 3));
    assert!(Solution::can_construct("true".to_string(), 4));
    assert!(!Solution::can_construct("cr".to_string(), 7));
}
