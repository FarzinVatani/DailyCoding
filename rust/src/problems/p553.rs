#[allow(dead_code)]
fn get_number_of_columns_to_remove(matrix: Vec<Vec<char>>) -> usize {
    let row = matrix.len();
    if row <= 1 {
        return 0;
    }

    let col = matrix[0].len();

    let mut counter = 0;
    for i in 0..col {
        for j in 0..(row - 1) {
            if matrix[j][i] > matrix[j + 1][i] {
                counter += 1;
                break;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_columns_1() {
        let matrix = vec![
            vec!['c', 'b', 'a'],
            vec!['d', 'a', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let result = get_number_of_columns_to_remove(matrix);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_remove_columns_2() {
        let matrix = vec![vec!['c', 'b', 'a', 'f', 'h']];
        let result = get_number_of_columns_to_remove(matrix);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_remove_columns_3() {
        let matrix = vec![
            vec!['z', 'y', 'x'],
            vec!['w', 'v', 'u'],
            vec!['t', 's', 'r'],
        ];
        let result = get_number_of_columns_to_remove(matrix);

        assert_eq!(result, 3);
    }
}
