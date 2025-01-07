/// # 2. Add Two Numbers
///
/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
pub struct Solution;

/// From the problem's description
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    /// Solution from [here](https://leetcode.com/problems/add-two-numbers/solutions/469977/simple-rust-solution-0ms-2-1mb).
    /// Passed all tests, runtime 0ms
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }

    /// My solution, failed to pass test 1566/1569 due to overflow errors,
    /// stemming from naively trying to represent ```l1``` and ```l2``` as u64s. The final test cases get big enough to
    /// overflow ```u64::MAX```, or ```18,446,744,073,709,551,615```
    pub fn _old_add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::iter::successors;

        let list1: Vec<_> = successors(l1, |i| i.next.clone()).collect();
        let mut total1: u64 = 0;

        for (i, item) in list1.iter().enumerate() {
            total1 += item.val as u64 * (10u64.pow(i as u32))
        }

        let list2: Vec<_> = successors(l2, |i| i.next.clone()).collect();
        let mut total2: u64 = 0;

        for (i, item) in list2.iter().enumerate() {
            total2 += item.val as u64 * (10u64.pow(i as u32))
        }

        let tot = (total1 + total2).to_string();
        let mut total_final = tot.chars();

        let mut prev = ListNode::new(total_final.next().unwrap().to_string().parse().unwrap());

        for c in total_final {
            let v: i32 = c.to_string().parse().unwrap();
            let latest = ListNode {
                val: v,
                next: Some(Box::new(prev.clone())),
            };
            prev = latest;
        }

        Some(Box::new(prev))
    }
}

pub fn run() {
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode::new(3))),
                    val: 4,
                })),
                val: 2,
            })),
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode::new(4))),
                    val: 6,
                })),
                val: 5,
            }))
        ),
        Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(8))),
                val: 0,
            })),
            val: 7,
        }))
    );
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode { next: None, val: 0 })),
            Some(Box::new(ListNode { next: None, val: 0 }))
        ),
        Some(Box::new(ListNode { next: None, val: 0 }))
    );
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode { next: None, val: 9 })),
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode {
                                    next: Some(Box::new(ListNode {
                                        next: Some(Box::new(ListNode {
                                            next: Some(Box::new(ListNode { next: None, val: 9 })),
                                            val: 9,
                                        })),
                                        val: 9,
                                    })),
                                    val: 9,
                                })),
                                val: 9,
                            })),
                            val: 9,
                        })),
                        val: 9,
                    })),
                    val: 9,
                })),
                val: 1,
            }))
        ),
        Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode {
                                    next: Some(Box::new(ListNode {
                                        next: Some(Box::new(ListNode {
                                            next: Some(Box::new(ListNode { next: None, val: 1 })),
                                            val: 0,
                                        })),
                                        val: 0,
                                    })),
                                    val: 0,
                                })),
                                val: 0,
                            })),
                            val: 0,
                        })),
                        val: 0,
                    })),
                    val: 0,
                })),
                val: 0,
            })),
            val: 0,
        }))
    );
}
