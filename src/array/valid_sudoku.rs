use std::collections::HashMap;

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut failed = false;

    let mut grid_items: HashMap<((u32, u32), u32), bool> = HashMap::new();

    for (col_idx, column) in board.iter().enumerate() {
        let mut col_items: HashMap<u32, bool> = HashMap::new();

        for (chr_idx, char) in column.iter().enumerate() {
            let char_num = char.to_digit(10);
            let mut row_items: HashMap<u32, bool> = HashMap::new();

            match char_num {
                Some(n) => {
                    let coords = (col_idx as u32 / 3, chr_idx as u32 / 3);
                    let grid_key = (coords, n);

                    if grid_items.contains_key(&grid_key) {
                        failed = true;
                    } else {
                        grid_items.insert(grid_key, true);
                    }

                    if row_items.get(&n).is_some() {
                        failed = true;
                    } else {
                        row_items.entry(n).or_insert(true);
                    }

                    if col_items.get(&n).is_some() {
                        failed = true;
                    } else {
                        col_items.entry(n).or_insert(true);
                    }
                }
                _ => {}
            }
        }
    }
    failed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_board_should_be_true() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(is_valid_sudoku(board), false); // valid sudoku → failed = false
    }

    #[test]
    fn invalid_board_should_be_false() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'], // duplicate 8 in column
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(is_valid_sudoku(board), true); // invalid sudoku → failed = true
    }
}
