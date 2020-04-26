use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::{fibonacci, climb_stairs};
    
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