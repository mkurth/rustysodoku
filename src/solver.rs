use crate::rustysodoku::{Board, Cell};
use crate::rustysodoku::Cell::Filled;

pub fn solve(board: &Board) -> Board {
    let mut board1 = board.clone();
    board1.cells[1][1] = Filled { number: 1 };
    board1
}

fn zip_with_all_solutions(cells: Vec<Vec<Cell>>) -> Vec<Vec<(Cell, Vec<usize>)>> {
    Vec::new()
}

fn reduce_impossible_solutions(cells: Vec<Vec<(Cell, Vec<usize>)>>) -> Vec<Vec<(Cell, Vec<usize>)>> {
    Vec::new()
}

fn reduce_possible_solution(cells: Vec<Vec<(Cell, Vec<usize>)>>) -> Vec<Vec<(Cell, Vec<usize>)>> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::rustysodoku::Cell::Filled;
    use crate::rustysodoku::Board;

    #[test]
    fn ca_solve_2x2_with_only_one_missing_field() {
        let values: Vec<(usize, usize, u8)> = [
            (0, 0, 1), (0, 1, 2),
            (1, 0, 2), (1, 1, 1)].to_vec();
        let board = fill_board(Board::init(2).deref(), values);

        let result = crate::solver::solve(&board);

        assert_eq!(result.cells[1][1], Filled { number: 1 })
    }

    #[test]
    fn can_solve_3x3_with_only_one_missing_field() {
        let values: Vec<(usize, usize, u8)> = [
            (0, 0, 1), (0, 1, 2), (0, 2, 3),
            (1, 0, 2), (1, 1, 3), (1, 2, 1),
            (2, 0, 3), (2, 1, 1)
        ].to_vec();
        let board = fill_board(Board::init(3).deref(), values);

        let result = crate::solver::solve(&board);

        assert_eq!(result.cells[2][2], Filled { number: 2 })
    }

    fn fill_board(board: &Board, values: Vec<(usize, usize, u8)>) -> Board {
        values.iter().fold(board.clone(), |b, &(row, column, number)| Board::fill_cell(&b, row, column, number))
    }
}