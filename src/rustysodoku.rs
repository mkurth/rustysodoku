use crate::rustysodoku::Cell::Empty;
use crate::rustysodoku::Cell::Filled;

#[derive(Clone, Hash, Debug, PartialEq, Copy)]
enum Cell {
    Empty {},
    Filled { number: u8 },
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn empty() -> Box<Board> {
        Box::from(Board { cells: Vec::new() })
    }
    pub fn init(size: u8) -> Box<Board> {
        let mut cells = Vec::new();
        for _row_idx in 0..size {
            let mut row: Vec<Cell> = Vec::new();
            for _column_idx in 0..size {
                row.push(Empty{})
            }
            cells.push(row)
        }
        Box::from(Board { cells })
    }
    pub fn default9x9() -> Box<Board> {
        crate::rustysodoku::Board::init(9)
    }

    pub fn fill_cell(board: &Board, row: usize, column: usize, number: u8) -> Board {
        let mut x = board.clone();
        x.cells[row][column] = Filled { number };
        x
    }
}

pub mod solver {
    use crate::rustysodoku::Board;

    pub fn solve(board: &Board) -> Board {
        board.clone()
    }
}

pub mod printer {
    use crate::rustysodoku::{Board, Cell};
    use crate::rustysodoku::Cell::{Filled, Empty};

    pub fn print_board(board: &Board) -> String {
        let length = board.cells.len();
        let horizontal_separator =
            &("\n".to_string() + &(0..length + 4).map(|_| "-").collect::<String>() + &"\n".to_string());
        horizontal_separator.to_string() +
            &board
                .cells
                .iter()
                .map(|row: &Vec<Cell>| print_row(row))
                .collect::<Vec<String>>()
                .chunks(3)
                .map(|chunk| chunk.join("\n"))
                .collect::<Vec<String>>()
                .join(horizontal_separator) + &horizontal_separator.to_string()
    }

    fn print_row(cells: &Vec<Cell>) -> String {
        let string_values = cells.iter().map(|cell: &Cell| match cell {
            Filled { number } => number.to_string(),
            Empty{} => " ".to_string(),
        });
        let row_content = string_values
            .collect::<Vec<String>>()
            .chunks(3)
            .map(|chunk| chunk.join(""))
            .collect::<Vec<String>>()
            .join("|");

        "|".to_owned() + &row_content + &"|".to_owned()
    }
}
