// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_same(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (Some(_), None) | (None, Some(_)) => false,
                (None, None) => true,
                (Some(l), Some(r)) => {
                    l.borrow().val == r.borrow().val
                        && is_same(&l.borrow().left, &r.borrow().right)
                        && is_same(&l.borrow().right, &r.borrow().left)
                }
            }
        }
        if let Some(n) = root {
            is_same(&n.borrow().left, &n.borrow().right)
        } else {
            true
        }
    }
}

#[test]
fn test() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let l2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let r2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let ll3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let lr3 = Rc::new(RefCell::new(TreeNode::new(4)));
    let rl3 = Rc::new(RefCell::new(TreeNode::new(4)));
    let rr3 = Rc::new(RefCell::new(TreeNode::new(3)));

    l2.borrow_mut().left = Some(Rc::clone(&ll3));
    l2.borrow_mut().right = Some(Rc::clone(&lr3));
    r2.borrow_mut().left = Some(Rc::clone(&rl3));
    r2.borrow_mut().right = Some(Rc::clone(&rr3));

    root.borrow_mut().left = Some(Rc::clone(&l2));
    root.borrow_mut().right = Some(Rc::clone(&r2));

    assert_eq!(Solution::is_symmetric(Some(root)), true);
}
