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