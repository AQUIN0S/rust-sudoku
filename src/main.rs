mod grid;
mod puzzle;

use grid::SudokuGrid;

fn main() {
    let sudoku = SudokuGrid::generate_example_solution();
    println!("Here's an example sudoku:\n");
    println!("{}", sudoku);
}
