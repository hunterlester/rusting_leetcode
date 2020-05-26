use std::collections::HashSet;

fn is_valid_sudoku(board: Vec<Vec<&str>>) -> bool {
    const BOARD_INDICES: [(usize, usize); 9] = [(0,0), (0,3), (0, 6), (3, 0), (3, 3), (3, 6), (6, 0), (6, 3), (6, 6)];
    let mut row_set: HashSet<&str> = HashSet::new();
    let mut col_set: HashSet<&str> = HashSet::new();
    let mut box_set: HashSet<&str> = HashSet::new();
    for (box_row_index, box_col_index) in &BOARD_INDICES {
        for row_index in *box_row_index..*box_row_index + 3 {
            for col_index in *box_col_index..*box_col_index + 3 {
                let board_value = board[row_index][col_index];
                if board_value != "." {
                    if let false = box_set.insert(board[row_index][col_index]) {
                        return false;
                    }
                }
            }
        }
        box_set.clear();
    }

    for row_index in 0..9 {
        for col_index in 0..9 {
            if board[row_index][col_index] != "." {
                if let false = row_set.insert(board[row_index][col_index]) {
                    return false;
                }
            }
            if board[col_index][row_index] != "." {
                if let false = col_set.insert(board[col_index][row_index]) {
                    return false;
                }
            }
        }
        row_set.clear();
        col_set.clear();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{is_valid_sudoku};

    #[test]
    fn test_valid_sudoku() {
        let board = vec![
          vec!["5","3",".",".","7",".",".",".","."],
          vec!["6",".",".","1","9","5",".",".","."],
          vec![".","9","8",".",".",".",".","6","."],
          vec!["8",".",".",".","6",".",".",".","3"],
          vec!["4",".",".","8",".","3",".",".","1"],
          vec!["7",".",".",".","2",".",".",".","6"],
          vec![".","6",".",".",".",".","2","8","."],
          vec![".",".",".","4","1","9",".",".","5"],
          vec![".",".",".",".","8",".",".","7","9"]
        ];
        assert!(is_valid_sudoku(board));
        let false_board = vec![
          vec!["8","3",".",".","7",".",".",".","."],
          vec!["6",".",".","1","9","5",".",".","."],
          vec![".","9","8",".",".",".",".","6","."],
          vec!["8",".",".",".","6",".",".",".","3"],
          vec!["4",".",".","8",".","3",".",".","1"],
          vec!["7",".",".",".","2",".",".",".","6"],
          vec![".","6",".",".",".",".","2","8","."],
          vec![".",".",".","4","1","9",".",".","5"],
          vec![".",".",".",".","8",".",".","7","9"]
        ];
        assert_eq!(is_valid_sudoku(false_board), false);
    }
}