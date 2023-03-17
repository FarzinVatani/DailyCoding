#[allow(dead_code)]
pub fn multiply_except_i(numbers: Vec<i32>) -> Vec<i32> {
    let length = numbers.len();

    let mut forward_multiplyer = 1;
    let mut backward_multiplyer = 1;
    let mut multiplied_numbers = vec![1; length];

    for (index, number) in numbers.iter().enumerate() {
        multiplied_numbers[index] *= forward_multiplyer;
        forward_multiplyer *= number;
    }

    for (index, number) in numbers.iter().rev().enumerate() {
        multiplied_numbers[length - index - 1] *= backward_multiplyer;
        backward_multiplyer *= number;
    }

    multiplied_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_except_i() {
        let numbers = vec![1, 2, 3, 4, 5];
        let expected = vec![120, 60, 40, 30, 24];

        let result = multiply_except_i(numbers);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiply_except_i_2() {
        let numbers = vec![3, 2, 1];
        let expected = vec![2, 3, 6];

        let result = multiply_except_i(numbers);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiply_except_i_empty_list() {
        let numbers = vec![];
        let expected = vec![];

        let result = multiply_except_i(numbers);

        assert_eq!(result, expected);
    }
}
