/// # 2116. Check if a Parentheses String Can Be Valid
/// *Daily Challenge 2025-01-12*
///
/// A parentheses string is a non-empty string consisting only of ```'('``` and ```')'```.
/// It is valid if any of the following conditions is true:
///
/// * It is ```()```.
/// * It can be written as ```AB``` (```A``` concatenated with ```B```), where ```A``` and ```B``` are valid parentheses strings.
/// * It can be written as ```(A)```, where ```A``` is a valid parentheses string.
///
/// You are given a parentheses string ```s``` and a string ```locked```, both of length ```n```. ```locked``` is a binary string
/// consisting only of ```'0'```s and ```'1'```s. For each index ```i``` of ```locked```,
///
/// * If ```locked[i]``` is ```'1'```, you cannot change ```s[i]```.
/// * But if ```locked[i]``` is ```'0'```, you can change ```s[i]``` to either ```'('``` or ```')'```.
///
/// Return ```true``` if you can make ```s``` a valid parentheses string. Otherwise, return ```false```.
pub struct Solution;

impl Solution {
    // some notes on the problem: i could probably just count the number of '('s or ')'s
    // and make sure they can be made the same
    // ig by tracking the number of each that can be changed
    // '(' = open = op, ')' = close = cl
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            // odd length means it's invalid automatically
            return false;
        }
        let mut c_op: i32 = 0;
        let mut c_cl: i32 = 0;
        let mut op_unlocked: i32 = 0;
        let mut cl_unlocked: i32 = 0;

        let s: Vec<char> = s.chars().collect();
        let locked: Vec<char> = locked.chars().collect();

        s.into_iter()
            .enumerate()
            .for_each(|(i, paren)| match paren {
                '(' => {
                    c_op += 1;
                    if locked[i] == '0' {
                        op_unlocked += 1;
                    }
                }
                _ => {
                    c_cl += 1;
                    if locked[i] == '0' {
                        cl_unlocked += 1;
                    }
                }
            });

        if c_op == c_cl {
            return true;
        }

        if c_op > c_cl {
            if (c_op - c_cl) >= (((op_unlocked - cl_unlocked) / 2) as f32).ceil() as i32
                && ((op_unlocked - cl_unlocked) / 2) != 0
            {
                return true;
            }
        } else if (c_cl - c_op) >= (((cl_unlocked - op_unlocked) / 2) as f32).ceil() as i32
            && ((cl_unlocked - op_unlocked) / 2) != 0
        {
            return true;
        }

        false
    }
}

pub fn run() {
    // didn't use assert_eq! because can_be_valid already returns a bool
    assert!(Solution::can_be_valid(
        "))()))".to_string(),
        "010100".to_string()
    ));
    assert!(Solution::can_be_valid(
        "()()".to_string(),
        "0000".to_string()
    ));
    assert!(!Solution::can_be_valid(")".to_string(), "0".to_string()));
    assert!(!Solution::can_be_valid(
        "))))(())((()))))((()((((((())())((()))((((())()()))(()".to_string(),
        "101100101111110000000101000101001010110001110000000101".to_string()
    ))
}
