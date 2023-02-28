struct Solution {}

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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();
        let mut seen = HashMap::new();
        fn walk(node: &Node, map: &mut HashMap<String, i32>, result: &mut Vec<Node>) -> String {
            if let Some(n) = node {
                let s = format!(
                    "{},{},{}",
                    n.borrow().val,
                    walk(&n.borrow().left, map, result),
                    walk(&n.borrow().right, map, result),
                );
                if let Some(v) = map.get_mut(&s) {
                    if *v == 1 {
                        result.push(node.clone());
                    }
                    *v += 1;
                } else {
                    map.insert(s.clone(), 1);
                }
                s
            } else {
                String::from("n")
            }
        }

        walk(&root, &mut seen, &mut result);
        result
    }
}

#[test]
fn test_find_duplicate_subtrees() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let layer1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let layer1_2 = Rc::new(RefCell::new(TreeNode::new(3)));
    let layer2_1 = Rc::new(RefCell::new(TreeNode::new(4)));
    layer1_1.borrow_mut().left = Some(Rc::clone(&layer2_1));
    let layer2_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let layer2_3 = Rc::new(RefCell::new(TreeNode::new(4)));
    layer1_2.borrow_mut().left = Some(Rc::clone(&layer2_2));
    layer1_2.borrow_mut().right = Some(Rc::clone(&layer2_3));
    let layer3_1 = Rc::new(RefCell::new(TreeNode::new(4)));
    layer2_2.borrow_mut().left = Some(Rc::clone(&layer3_1));
    root.borrow_mut().left = Some(Rc::clone(&layer1_1));
    root.borrow_mut().right = Some(Rc::clone(&layer1_2));
    // println!("{}", *layer1_1.borrow() == *layer2_2.borrow());
    // println!(
    //     "{:?}",
    //     Rc::clone(&root).borrow().left
    //         == Rc::clone(&root)
    //             .borrow()
    //             .right
    //             .as_ref()
    //             .unwrap()
    //             .borrow()
    //             .left
    // );
    let v = Solution::find_duplicate_subtrees(Some(Rc::clone(&root)));
    println!("{v:?}");
}
