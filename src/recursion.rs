use std::cell::RefCell;
use std::rc::Rc;

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

#[cfg(test)]
mod tests {
    use super::{fibonacci};
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }
}