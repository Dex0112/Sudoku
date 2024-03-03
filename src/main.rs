use sudoku::Sudoku;

mod sudoku;
mod sudoku_validator;
mod sudoku_solver;


fn main() {
    let sudoku: Sudoku = Sudoku::new(sudoku::SudokuOptions::New(9));
    println!("{}", sudoku.to_string());
}
