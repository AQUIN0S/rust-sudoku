mod grid;
mod puzzle;

use grid::SudokuGrid;

fn main() {
    let sudoku = SudokuGrid::generate_example_solution();
}
