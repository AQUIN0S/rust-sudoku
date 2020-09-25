/// A value between 1-9 inclusive, representing a number on a 9x9 sudoku board.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Value(u8);

impl Value {
    pub fn new(value: u8) -> Value {
        if value > 0 && value < 10 {
            return Value(value)
        }
        panic!("Cannot create value {}, number must be 1-9 inclusive.", value);
    }

    pub fn set(&mut self, value: u8) {
        if value > 0 && value < 10 {
            self.0 = value;
        } else {
            panic!("Cannot set value to {}, number must be 1-9 inclusive.", value);
        }
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(Value::new(1), Value(1));
    }

    #[test]
    #[should_panic(expected = "Cannot create value 0, number must be 1-9 inclusive.")]
    fn panic_creating_0() {
        assert_eq!(Value::new(0), Value(0));
    }

    #[test]
    #[should_panic(expected = "Cannot create value 10, number must be 1-9 inclusive.")]
    fn panic_creating_10() {
        assert_eq!(Value::new(10), Value(10));
    }

    #[test]
    fn compare() {
        let lhs = Value::new(1);
        let rhs = Value::new(1);
        assert_eq!(lhs, rhs);

        let rhs = Value::new(2);
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn compare_u8() {
        let lhs = Value::new(1);
        let rhs: u8 = 1;
        assert_eq!(lhs, rhs);

        let rhs: u8 = 2;
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn set() {
        let mut lhs = Value::new(1);
        assert_eq!(lhs, Value(1));

        lhs.set(5);
        assert_eq!(lhs, Value(5));
    }

    #[test]
    #[should_panic(expected = "Cannot set value to 0, number must be 1-9 inclusive.")]
    fn panic_setting_0() {
        let mut lhs = Value::new(1);
        assert_eq!(lhs, Value(1));

        lhs.set(0);
        assert_eq!(lhs, Value(10));
    }

    #[test]
    #[should_panic(expected = "Cannot set value to 10, number must be 1-9 inclusive.")]
    fn panic_setting_10() {
        let mut lhs = Value::new(1);
        assert_eq!(lhs, Value(1));

        lhs.set(10);
        assert_eq!(lhs, Value(10));
    }

    #[test]
    fn get() {
        let value = Value::new(4);
        let comp_val: u8 = 4;
        assert_eq!(value.get(), comp_val);
    }
}