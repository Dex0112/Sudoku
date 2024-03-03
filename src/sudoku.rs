use std::{usize, vec};

pub enum SudokuError {
    InvalidMove,
    InvalidValue,
    OutOfBounds,
}

pub enum SudokuOptions {
    New(usize),
    Pregenerated(Vec<Vec<usize>>),
}

pub struct Sudoku {
    size: usize,
    grid: Vec<Vec<usize>>,
}

impl ToString for Sudoku {
    fn to_string(&self) -> String {
        todo!("Make pretty grid graphic for cli");
    }
}

impl Sudoku {
    pub fn new(option: SudokuOptions) -> Sudoku {
        match option {
            SudokuOptions::New(size) => {

                return Sudoku {
                    size,
                    grid: Sudoku::generate_board(size)
                };
            }

            SudokuOptions::Pregenerated(grid) => {
                //Do I check if it is a valid sudoku? Do I Error?
                return Sudoku {
                    size: grid.len(),
                    grid,
                }
            }
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize, value: usize) -> Result<(), SudokuError> {
        if x >= self.grid.len() {
            return Err(SudokuError::OutOfBounds);
        }

        if y >= self.grid[x].len() {
            return Err(SudokuError::OutOfBounds);
        }

        if value > self.size {
            return Err(SudokuError::InvalidValue);
        }

        self.grid[x][y] = value;

        return Ok(());
    }

    fn generate_board(size: usize) -> Vec<Vec<usize>> {

        
        return vec![vec![0; size]; size];
    }
}
