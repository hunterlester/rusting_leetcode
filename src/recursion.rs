use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn build_bst(array: &Vec<Option<i32>>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if index <= array.len() - 1 {
        let next_left_child_index = (index*2) + 1;
        let next_right_child_index = (index*2) + 2;
        if let Some(value) = array[index] {
            Some(Rc::new(RefCell::new(TreeNode {
                val: value,
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

fn fibonacci(n: i32) -> i32 {
    let mut cache: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0, 1]));

    fn recurse(n: i32, cache: Rc<RefCell<Vec<i32>>>) -> i32 {
        if n < 2 {
            return n;
        }
        if let Some(cached_value) = cache.borrow().get(n as usize) {
            return *cached_value;
        }
        let value = recurse(n - 2, Rc::clone(&cache)) + recurse(n - 1, Rc::clone(&cache));
        cache.borrow_mut().insert(n as usize, value);
        return value;
    }
    recurse(n, Rc::clone(&cache))
}

fn climb_stairs(n: i32) -> i32 {
    let mut cache: Rc<RefCell<HashMap<usize, i32>>> = Rc::new(RefCell::new(HashMap::new()));
    fn recurse(step: usize, n:i32, cache: Rc<RefCell<HashMap<usize, i32>>>) -> i32 {
        if step as i32 > n {
            return 0;
        }
        if step as i32 == n {
            return 1;
        }
        if let Some(cached_value) = cache.borrow().get(&step) {
            return *cached_value;
        }
        let value = recurse(step + 1, n, Rc::clone(&cache)) + recurse(step + 2, n, Rc::clone(&cache));
        cache.borrow_mut().insert(step, value);
        return value;
    }
    recurse(0, n, Rc::clone(&cache))
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left_height = if let Some(left_child) = node.borrow().left.as_ref() {
            max_depth(Some(Rc::clone(left_child)))
        } else {0};
        let right_height = if let Some(right_child) = node.borrow().right.as_ref() {
            max_depth(Some(Rc::clone(right_child)))
        } else {0};
        max(left_height, right_height) + 1
    } else {
        0
    }
}

fn my_pow(x: f64, n: i32) -> f64 {
    fn fast_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1_f64;
        }
        let half = fast_pow(x, n / 2);
        if n % 2 == 0 {
            half * half
        } else {
            half * half * x
        }
    }
    let mut product = fast_pow(x, n.abs());
    if n < 0 {
        1 as f64 / product
    } else {
        product as f64
    }
}

#[cfg(test)]
mod tests {
    use super::{fibonacci, climb_stairs, max_depth, build_bst, my_pow};

    #[test]
    fn test_pow() {
        assert_eq!(my_pow(2 as f64, 10), 1024 as f64);
        assert_eq!(my_pow(2 as f64, -2), 0.25);
        assert_eq!(my_pow(2 as f64, 0), 1 as f64);
        assert_eq!(my_pow(-2 as f64, 3), -8 as f64);
        assert_eq!(my_pow(-2 as f64, 2), 4 as f64);
        assert_eq!(my_pow(-2 as f64, -2), 0.25);
        assert_eq!(my_pow(0.00001, 2147483647), 0_f64);
    }

    #[test]
    fn test_max_depth() {
        let mut bst_values = vec![Some(3), Some(9), Some(20), None, None, Some(15) , Some(7)];
        let mut bst = build_bst(&bst_values, 0);
        assert_eq!(max_depth(bst), 3);
        bst_values = vec![Some(1), Some(2), Some(3), Some(4) , Some(5)];
        bst = build_bst(&bst_values, 0);
        assert_eq!(max_depth(bst), 3);
        assert_eq!(max_depth(None), 0);
    }
    
    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }
}