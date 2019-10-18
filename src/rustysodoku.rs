use crate::rustysodoku::Cell::Empty;
use crate::rustysodoku::Cell::Filled;

#[derive(Clone, Hash, Debug, PartialEq, Copy)]
pub enum Cell {
    Empty {},
    Filled { number: u8 },
}

#[derive(Clone, Hash, Debug, PartialEq)]
pub struct Board {
    pub cells: Vec<Vec<Cell>>,
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



pub mod printer {

}
