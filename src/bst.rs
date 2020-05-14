#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

type NodeOption = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: NodeOption,
  pub right: NodeOption,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct BSTIterator {
    pub current_index: usize,
    pub inorder_values: Vec<i32>,
}

impl BSTIterator {
    fn inorder_traverse(root: NodeOption) -> Vec<i32> {
        if let Some(node) = root {
            let left_child = if let Some(left_node) = node.borrow().left.as_ref() {
                Self::inorder_traverse(Some(Rc::clone(left_node)))
            } else { vec![] };
            let right_child = if let Some(right_node) = node.borrow().right.as_ref() {
                Self::inorder_traverse(Some(Rc::clone(right_node)))
            } else { vec![] };
           vec![left_child, vec![node.borrow().val], right_child].concat()
        } else {
            vec![]
        }
    }

    fn new(root: NodeOption) -> Self {
        let inorder_values: Vec<i32> = Self::inorder_traverse(root);
        Self {
            current_index: 0,
            inorder_values,
        }
    }
    
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
         let index = self.current_index;
         self.current_index += 1;
         self.inorder_values[index] 
    }
    
    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        if let Some(_value) = self.inorder_values.get(self.current_index) {
            true
        } else {
            false
        }
    }
}

fn build_bst(array: &Vec<i32>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index <= array.len() - 1 {
        let next_left_child_index = (index*2) + 1;
        let next_right_child_index = (index*2) + 2;
        if let Some(value) = array.get(index) {
            Some(Rc::new(RefCell::new(TreeNode {
                val: *value,
                left: build_bst(&array, next_left_child_index),
                right: build_bst(&array, next_right_child_index),
            })))
        } else {
            None
        }
    } else {
        None
    }
}

fn invert_tree(root: NodeOption) -> NodeOption {
    if let Some(node) = root {
        let mut borrowed_node = node.borrow_mut();
        let left_child = if let Some(left) = borrowed_node.left.as_ref() {
            invert_tree(Some(Rc::clone(&left)))
        } else {None};
        let right_child = if let Some(right) = borrowed_node.right.as_ref() {
            invert_tree(Some(Rc::clone(&right)))
        } else {None};
        borrowed_node.left = right_child;
        borrowed_node.right = left_child;
        Some(Rc::clone(&node))
    } else {None}
}

#[cfg(test)]
mod tests {
    use super::{invert_tree, build_bst};

    #[test]
    fn test_invert_tree() {
        let bst = build_bst(&vec![4,2,7,1,3,6,9], 0);
        let expected = build_bst(&vec![4,7,2,9,6,3,1], 0);
        assert_eq!(invert_tree(bst), expected)
    }
}