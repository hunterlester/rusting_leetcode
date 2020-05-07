use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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

fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut a_index = 0;
    let mut b_index = 0;
    let mut sorted: Vec<i32> = Vec::new();
    while a_index < a.len() && b_index < b.len() {
        if a[a_index] < b[b_index] {
            sorted.push(a[a_index]);
            a_index += 1;
        } else {
            sorted.push(b[b_index]);
            b_index += 1;
        }
    }
    if a_index < a.len() {
        sorted.extend_from_slice(&a[a_index..]);
    }
    if b_index < b.len() {
        sorted.extend_from_slice(&b[b_index..]);
    }
    sorted
}

fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }
    let pivot = nums.len() / 2;
    let left_sub: Vec<i32> = merge_sort(nums[0..pivot].to_vec());
    let right_sub: Vec<i32> = merge_sort(nums[pivot..].to_vec());
    merge(left_sub, right_sub)
}

fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    merge_sort(nums)
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![]; 
    let mut current_node: Option<Rc<RefCell<TreeNode>>> = None;
    let mut prev_value: Option<i32> = None;
    if let Some(node) = root {
        current_node = Some(Rc::clone(&node));
    }
    while current_node != None || stack.len() != 0 {
        while let Some(node) = current_node {
            stack.push(Some(Rc::clone(&node)));
            current_node = if let Some(left_child) = node.borrow().left.as_ref() {
                Some(Rc::clone(left_child))
            } else {None};
        }

        if let Some(Some(node)) = stack.pop() {
            if let Some(previous_value) = prev_value {
                if node.borrow().val <= previous_value {
                    return false;
                }
            }
            prev_value = Some(node.borrow().val);
            current_node = if let Some(right_child) = node.borrow().right.as_ref() {
                Some(Rc::clone(right_child))
            } else {None};
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{sort_array, is_valid_bst, build_bst};

    #[test]
    fn test_is_valid_bst() {
        let mut bst_values = vec![Some(2), Some(1), Some(3)];
        let mut bst = build_bst(&bst_values, 0);
        assert_eq!(is_valid_bst(bst), true);
        bst_values = vec![Some(5), Some(2), Some(9), Some(1) , Some(3), Some(8), Some(11)];
        bst = build_bst(&bst_values, 0);
        assert_eq!(is_valid_bst(bst), true);
        bst_values = vec![Some(10), Some(5), Some(15), None, None, Some(6), Some(20)];
        bst = build_bst(&bst_values, 0);
        assert_eq!(is_valid_bst(bst), false);
        bst_values = vec![Some(1), Some(1)];
        bst = build_bst(&bst_values, 0);
        assert_eq!(is_valid_bst(bst), false);
    }

    #[test]
    fn test_sort_array() {
        let array = vec![7, 3, 8, 1, 4];
        let expected = vec![1, 3, 4, 7, 8];
        assert_eq!(sort_array(array), expected);
        assert_eq!(sort_array(vec![]), vec![]);
    }
}