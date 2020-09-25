mod value;
pub use value::Value;

/// Represents a grid item in a sudoku - essentially one square. An item could contain a list of values which the player thinks may be in the square,
/// or may contain a value between 1-9 inclusive.
///
/// The value may be fixed or not - a fixed item represents an unchanging value, such as a number that
/// was placed by the computer at the start of a sudoku puzzle, which is definitely the right value and cannot be changed.
///
/// Contrarily, an unfixed value is one that the player put in, and they can change the value if they change their mind.
#[derive(Copy, Clone, Debug, Eq)]
pub enum Item {
    Number {
        fixed: bool,
        value: Value,
    },
    Notes([Option<Value>; 9]),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {

        match self {
            Item::Number { value: lhs, .. } => {
                if let Item::Number { value: rhs, .. } = other {
                    lhs == rhs
                } else {
                    false
                }
            },
            Item::Notes(note_values) => {
                if let Item::Notes(other_note_values) = other {
                    for (i, value) in note_values.iter().enumerate() {
                        if value != &other_note_values[i] {
                            return false;
                        }
                    }
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

/// It just makes sense to me to make it easy to compare an `Item::Number` variant to a simple u8. An `Item::Notes` variant will automatically return
/// `false`.
impl PartialEq<u8> for Item {
    fn eq(&self, other: &u8) -> bool {
        if let Item::Number { value, .. } = self {
            value == other
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(Item::Number { value: Value::new(1), fixed: true }, 1);
    }
}
