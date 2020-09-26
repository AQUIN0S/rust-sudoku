mod value;
pub use value::Value;

/// Represents a grid item in a sudoku - essentially one square. An item could contain a list of values which the player thinks may be in the square (`Notes`),
/// or may contain a value between 1-9 inclusive (`Number`).
///
/// The `Number` value may be fixed or not - a fixed item represents an unchanging value, such as a number that
/// was placed by the computer at the start of a sudoku puzzle, which is definitely the right value and cannot be changed.
///
/// Contrarily, an unfixed value is one that the player put in, and they can change the value if they change their mind.
#[derive(Copy, Clone, Debug, Eq)]
#[allow(dead_code)]
pub enum Item {
    Number {
        fixed: bool,
        value: Value,
    },
    Notes([Option<Value>; 9]),
}

#[allow(dead_code)]
impl Item {
    pub fn set_fixed_number(&mut self, value: u8) {
        let (value, fixed) = (Value::new(value), true);
        *self = Item::Number { value, fixed };
    }
 
    pub fn toggle_number(&mut self, new_value: u8, force_change: bool) {
        match self {
            Item::Number { value, fixed } => {
                if force_change || !*fixed {
                    if *value == new_value {
                        *self = Item::Notes([None; 9]);
                    } else {
                        value.set(new_value);
                    }
                }
            },
            Item::Notes(..) => {
                *self = Item::Number { value: Value::new(new_value), fixed: false };
            },
        }
    }

    pub fn toggle_note(&mut self, value: u8) {
        match self {
            Item::Number { .. } => {
                *self = Item::Notes([None; 9]);
                if let Item::Notes(note_vals) = self {
                    note_vals.iter_mut().enumerate().for_each(|(index, note_value)| {
                        if value as usize == index + 1 {
                            *note_value = Some(Value::new(value));
                        }
                    });
                }
            },
            Item::Notes(note_vals) => {
                note_vals.iter_mut().enumerate().for_each(|(index, note_value)| {
                    if value as usize == index + 1 {
                        if *note_value == None {
                            *note_value = Some(Value::new(value));
                        } else {
                            *note_value = None;
                        }
                    }
                });
            },
        }
    }
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
    /// Returns `true` if an `Item::Number` variant's `value` field equates to the provided `u8` value.
    ///
    /// Returns `false` in any other case.
    fn eq(&self, other: &u8) -> bool {
        if let Item::Number { value, .. } = self {
            value == other
        } else {
            false
        }
    }
}

use std::fmt;

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Number { value, .. } => {
                write!(f, "{}", value.get())
            },
            Item::Notes(noted_values) => {
                let noted_values = noted_values.iter().map(|| {

                }).collect();
                write!(f, "[")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_notes() {
        let lhs = Item::Notes([None; 9]);
        let mut rhs = Item::Notes([None; 9]);

        assert_eq!(lhs, rhs);

        let lhs = Item::Notes([None, None, None, Some(Value::new(4)), None, None, None, None, None,]);
        rhs.toggle_note(4);
        assert_eq!(lhs, rhs);

        let lhs = Item::Notes([None; 9]);
        rhs.toggle_note(4);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn compare_number() {
        assert_eq!(Item::Number { value: Value::new(1), fixed: true }, 1);
    }
}
