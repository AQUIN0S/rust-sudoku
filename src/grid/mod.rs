mod item;

use item::Item;

#[derive(Debug)]
pub struct SudokuGrid {
    pub numbers: [[Item; 9]; 9],
}

// SudokuGrid {
//     numbers:
//     [
//         [ 1, 2, 3, 4, 5, 6, 7, 8, 9, ],
//         [ 4, 5, 6, 7, 8, 9, 1, 2, 3, ],
//         [ 7, 8, 9, 1, 2, 3, 4, 5, 6, ],
//         [ 2, 3, 4, 5, 6, 7, 8, 9, 1, ],
//         [ 5, 6, 7, 8, 9, 1, 2, 3, 4, ],
//         [ 8, 9, 1, 2, 3, 4, 5, 6, 7, ],
//         [ 3, 4, 5, 6, 7, 8, 9, 1, 2, ],
//         [ 6, 7, 8, 9, 1, 2, 3, 4, 5, ],
//         [ 9, 1, 2, 3, 4, 5, 6, 7, 8, ],
//     ]
// }

impl SudokuGrid {

    pub fn new() -> SudokuGrid {
        SudokuGrid {
            numbers: [[Item::Notes([None; 9]); 9]; 9],
        }
    }

    pub fn generate_example_solution() -> SudokuGrid {
        let mut example_grid = SudokuGrid::new();
        example_grid
    }

    pub fn check_guess(guess: &Item, solution: &Item) -> bool {
        if let Item::Number { fixed: true, .. } = solution {
            guess == solution
        } else {
            panic!("Solution value {:?} is not actually a fixed value item. Think hard about what you've done!", solution);
        }
    }
}

// use std::fmt::{ Display, Formatter, Result };
// use ansi_term::Colour;

// impl Display for SudokuGrid {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut sudoku_string = String::new();
//         let major_row_separator = Colour::Blue.bold().paint("+---+---+---+---+---+---+---+---+---+\n").to_string();
//         let coloured_intersection = Colour::Blue.bold().paint("+").to_string();
//         let major_column_separator = Colour::Blue.bold().paint("|").to_string();

//         for (row_index, row) in self.numbers.iter().enumerate() {
//             if row_index % 3 == 0 {
//                 sudoku_string.push_str(major_row_separator.as_str());
//             } else {
//                 sudoku_string.push_str(
//                     (
//                         format!(
//                             "{}---+---+---{}---+---+---{}---+---+---{}\n",
//                             coloured_intersection,
//                             coloured_intersection,
//                             coloured_intersection,
//                             coloured_intersection,
//                         )
//                     ).as_str()
//                 );
//             }

//             for (item_index, item) in row.iter().enumerate() {
//                 if item_index % 3 == 0 {
//                     let item_string = format!("{} {} ", major_column_separator.as_str(), item);
//                     sudoku_string.push_str(item_string.as_str());
//                 } else {
//                     let item_string = format!("| {} ", item);
//                     sudoku_string.push_str(item_string.as_str());
//                 }

//             }
//             sudoku_string.push_str((format!("{}\n", major_column_separator.as_str())).as_str());
//         }
//         sudoku_string.push_str(major_row_separator.as_str());

//         write!(f, "{}", sudoku_string)
//     }
// }
