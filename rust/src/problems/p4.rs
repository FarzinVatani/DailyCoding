fn find_min_positive(arr: &mut Vec<isize>) -> isize {
    let k = seperate_neg_and_pos_inplace(arr);

    for i in 0..k {
        let value_as_index = arr[i].abs() as usize;
        if value_as_index <= k {
            if arr[value_as_index - 1] > 0 {
                arr[value_as_index - 1] = -arr[value_as_index - 1];
            }
        }
    }

    let mut result: isize = k as isize;
    for i in 0..k {
        if arr[i] > 0 {
            result = i as isize;
            break;
        }
    }

    result + 1
}

fn seperate_neg_and_pos_inplace(arr: &mut Vec<isize>) -> usize {
    let array_length = arr.len();
    let mut i = 0;
    let mut j = array_length - 1;

    while i < j {
        while arr[i] > 0 {
            i += 1;
        }
        while arr[j] <= 0 {
            j -= 1;
        }

        if i < j {
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seperate_neg_and_pos() {
        let mut arr = vec![-2, 3, -1, 0, 1, 4, 5];
        let result = seperate_neg_and_pos_inplace(&mut arr);
        let expected: Vec<isize> = vec![5, 3, 4, 1, 0, -1, -2];

        assert_eq!(expected, arr);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_seperate_neg_and_pos_4_value() {
        let mut arr = vec![3, 4, -1, 1];
        let result = seperate_neg_and_pos_inplace(&mut arr);
        let expected: Vec<isize> = vec![3, 4, 1, -1];

        assert_eq!(expected, arr);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_find_min_positive() {
        let mut arr1 = vec![3, 4, -1, 1];
        let mut arr2 = vec![1, 2, 0];

        let result1 = find_min_positive(&mut arr1);
        let result2 = find_min_positive(&mut arr2);

        assert_eq!(result1, 2);
        assert_eq!(result2, 3);
    }
}
