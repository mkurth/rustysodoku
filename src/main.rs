use std::ops::Deref;

fn main() {
    let b = rustysodoku::rustysodoku::Board::default9x9();
    let c = rustysodoku::rustysodoku::Board::fill_cell(b.deref(), 1, 1, 6);
    println!("{}", rustysodoku::printer::print_board(&c));
}
