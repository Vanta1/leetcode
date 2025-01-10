/// # 1769. Minimum Number of Operations to Move All Balls to Each Box
/// *Daily Challenge 2025-01-06*
///
/// You have ```n``` boxes. You are given a binary string ```boxes``` of length ```n```,
/// where ```boxes[i]``` is ```'0'``` if the ```ith``` box is empty, and ```'1'``` if it contains one ball.
///
/// In one operation, you can move one ball from a box to an adjacent box.
/// Box ```i``` is adjacent to box ```j``` if ```abs(i - j) == 1```.
/// Note that after doing so, there may be more than one ball in some boxes.
///
/// Return an array answer of size ```n```, where ```answer[i]``` is the minimum number of operations needed
/// to move all the balls to the ```ith``` box.
///
/// Each ```answer[i]``` is calculated considering the initial state of the boxes.
pub struct Solution;

impl Solution {
    /// This solution is from the [editorial](https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/editorial/),
    /// and had a runtime of 19ms
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let l = boxes.len();
        let mut answer: Vec<i32> = vec![0; l];

        let mut balls_left = 0i32;
        let mut moves_left = 0i32;
        let mut balls_right = 0i32;
        let mut moves_right = 0i32;

        for i in 0..l {
            *answer.get_mut(i).unwrap() += moves_left;
            balls_left += (boxes.chars().nth(i).unwrap() == '1') as i32;
            moves_left += balls_left;

            let j = l - 1 - i;
            *answer.get_mut(j).unwrap() += moves_right;
            balls_right += (boxes.chars().nth(j).unwrap() == '1') as i32;
            moves_right += balls_right;
        }

        answer
    }

    /// Solution from my first attempt. this was accepted and passed all tests, but
    /// performance was very poor w/ runtime of 59ms
    pub fn _old_min_operations(boxes: String) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::with_capacity(boxes.len());
        for i in 0..boxes.len() {
            let mut total = 0i32;
            for (n, char) in boxes.chars().enumerate() {
                if char == '1' && i != n {
                    total += (n as i32 - i as i32).abs();
                }
            }
            answer.push(total);
        }
        answer
    }
}

pub fn run() {
    assert_eq!(Solution::min_operations("110".into()), vec![1, 1, 3]);
    assert_eq!(
        Solution::min_operations("001011".into()),
        vec![11, 8, 5, 4, 3, 4]
    );
}
