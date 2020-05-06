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

#[cfg(test)]
mod tests {
    use super::{sort_array};

    #[test]
    fn test_sort_array() {
        let array = vec![7, 3, 8, 1, 4];
        let expected = vec![1, 3, 4, 7, 8];
        assert_eq!(sort_array(array), expected);
        assert_eq!(sort_array(vec![]), vec![]);
    }
}