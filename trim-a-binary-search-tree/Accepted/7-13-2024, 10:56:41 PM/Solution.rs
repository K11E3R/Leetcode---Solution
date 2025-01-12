// https://leetcode.com/problems/trim-a-binary-search-tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            let val = node_ref.val;
            
            if val < low {
                return Self::trim_bst(node_ref.right.clone(), low, high);
            } else if val > high {
                return Self::trim_bst(node_ref.left.clone(), low, high);
            } else {
                node_ref.left = Self::trim_bst(node_ref.left.clone(), low, high);
                node_ref.right = Self::trim_bst(node_ref.right.clone(), low, high);
                return Some(node.clone());
            }
        }
        
        None
    }
}