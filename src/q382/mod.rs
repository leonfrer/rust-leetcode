use rand::Rng;

// Definition for singly-linked list.
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
struct Solution {
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use rand::prelude::*;
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut s = Solution { values: Vec::new() };
        if let Some(b) = head {
            s.values.push(b.val);
            let mut next = b.next;
            while let Some(bx) = next {
                s.values.push(bx.val);
                next = bx.next;
            }
        }
        s
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.values.len());
        self.values[i]
    }
}

#[test]
fn test() {
    let mut head = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let node3 = ListNode::new(3);
    node2.next = Some(Box::new(node3));
    head.next = Some(Box::new(node2));
    let s = Solution1::new(Some(Box::new(head)));
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
}

struct Solution1 {
    head: Option<Box<ListNode>>,
}

impl Solution1 {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution1 { head }
    }

    fn get_random(&self) -> i32 {
        let mut result = -1;
        let mut node = &self.head;
        let mut i = 0;
        let mut rng = rand::thread_rng();
        while let Some(b) = node {
            i += 1;
            node = &b.as_ref().next;
            if rng.gen_range(0.0..1.0) < 1.0 / i as f64 {
                result = b.as_ref().val;
            }
        }
        result
    }
}
