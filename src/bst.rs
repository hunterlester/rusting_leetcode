use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct BSTIterator {
    pub current_index: usize,
    pub inorder_values: Vec<i32>,
}

impl BSTIterator {
    fn inorder_traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
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
        if let Some(value) = self.inorder_values.get(self.current_index) {
            true
        } else {
            false
        }
    }
}
