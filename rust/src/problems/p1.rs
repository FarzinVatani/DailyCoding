use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_numbers(numbers: Vec<i32>, k: i32) -> bool {
    let mut cache = HashMap::<i32, i32>::new();

    for number in numbers {
        if cache.contains_key(&number) {
            println!("{}: {}", number, cache[&number]);
            return true;
        }

        cache.insert(k - number, number);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_numbers_return_false() {
        let numbers: Vec<i32> = vec![10, 15, 3, 7];
        let k: i32 = 20;

        let result = two_numbers(numbers, k);

        assert!(!result);
    }

    #[test]
    fn test_two_numbers_empty_return_false() {
        let numbers: Vec<i32> = vec![];
        let k: i32 = 20;

        let result = two_numbers(numbers, k);

        assert!(!result);
    }

    #[test]
    fn test_two_numbers_return_true() {
        let numbers: Vec<i32> = vec![10, 15, 3, 7];
        let k: i32 = 17;

        let result = two_numbers(numbers, k);

        assert!(result);
    }
}
