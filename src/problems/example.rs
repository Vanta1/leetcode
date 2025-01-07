#![allow(dead_code)] // don't copy this, it's here to suppress warnings about this unused example.

/// # Title goes here
/// *If it's a daily challenge, add that here with the format "Daily Challenge YYYY-MM-DD"*
///
/// Paste the problem description here, I like to add code blocks for formatting variables/parameters with \`\`\`
pub struct Solution;

impl Solution {
    /// Copy the solution impl from LeetCode here
    ///
    /// I like to add whether the solution was mine, and if not a link to where I got it from.
    /// Also the runtime from LeetCode after submitting it.
    pub fn example_fn(_some_stuff: Vec<i32>) -> Vec<u16> {
        unreachable!()
    }

    /// Unused solutions that I want to preserve have the prefix "_old_" on the identifier.
    pub fn _old_example_fn(_some_stuff: Vec<i32>) -> Vec<u16> {
        unreachable!()
    }
}

/// Add test cases here, I like to use ```assert_eq!()``` statements so I don't have to visually check test results.
pub fn run() {}
