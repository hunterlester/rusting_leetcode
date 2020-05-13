#![allow(dead_code)]

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

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let pivot = left + (right - left) / 2;
        if nums[pivot] == target {
            return pivot as i32;
        }
        if target < nums[pivot] && pivot > 0 {
            right = pivot - 1;
        } else {
            left = pivot + 1;
        }
    }
    return -1;
}


fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for vector in matrix {
        if target >= vector[0] && target <= vector[vector.len() - 1] {
            if binary_search(vector, target) > -1 {
                return true;
            }
        }
    }
    return false;
}

fn partition(nums: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot_value = nums[high as usize];
    let mut partition_index = low;
    for index in low..high {
        if nums[index as usize] < pivot_value {
            nums.swap(index as usize, partition_index as usize);
            partition_index += 1;
        }
    }
    nums.swap(partition_index as usize, high as usize);
    partition_index
}

/// Quick sort partition variant
// fn partition(nums: &mut Vec<i32>, mut low: i32, mut high: i32) -> i32 {
//     let pivot_index = (low + (high - low)) / 2; // low + high >>> 1
//     while low <= high {
//         while nums[low as usize] < nums[pivot_index as usize] {
//             low += 1;
//         }
//         while nums[high as usize] > nums[pivot_index as usize] {
//             high -= 1;
//         }
//         if low <= high {
//             nums.swap(low as usize, high as usize);
//             low += 1;
//             high -= 1;
//         }
//     }
//     low
// }

fn quick_sort_helper(mut nums: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let partition_index = partition(&mut nums, low, high);
        quick_sort_helper(&mut nums, low, partition_index - 1);
        quick_sort_helper(&mut nums, partition_index + 1, high);
    }
}

fn quick_sort(mut nums: &mut Vec<i32>) {
    let length = nums.len() as i32;
    quick_sort_helper(&mut nums, 0, length - 1);
}

#[cfg(test)]
mod tests {
    use super::{sort_array, is_valid_bst, build_bst, search_matrix, binary_search, quick_sort};

    #[test]
    fn test_quick_sort() {
        let mut array = vec![3, 7, 8, 1, 4];
        let expected = vec![1, 3, 4, 7, 8];
        quick_sort(&mut array);
        assert_eq!(array, expected);
    }

    #[test]
    fn test_binary_search() {
        let mut array = vec![1, 4, 7, 9, 12, 23, 27, 30];
        assert_eq!(binary_search(array, 27), 6);
        array = vec![5];
        assert_eq!(binary_search(array, 5), 0);
        array = vec![5];
        assert_eq!(binary_search(array, -5), -1);
        array = vec![-1, 0, 5];
        assert_eq!(binary_search(array, -1), 0);
    }

    #[test]
    fn test_search_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
          vec![1,   4,  7, 11, 15],
          vec![2,   5,  8, 12, 19],
          vec![3,   6,  9, 16, 22],
          vec![10, 13, 14, 17, 24],
          vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(search_matrix(matrix, 5), true);
    }

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