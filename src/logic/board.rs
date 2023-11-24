use bevy::prelude::*;

/// Core data structures and logic for the Sudoku game board
use self::marks::{CenterMarks, CornerMarks};

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Component)]
pub struct Coordinates {
    /// Between 1 and 9, counted from top to bottom
    pub row: u8,
    /// Between 1 and 9, counted from left to right
    pub column: u8,
    /// Squares are counted from 1 to 9 starting at the top left,
    /// in standard left-to-right reading order
    ///
    /// The standard term for the 3x3 box a cell is in is `box`,
    /// but that's a reserved word in Rust
    pub square: u8,
}

impl Coordinates {
    /// Computes which 3x3 square a cell is in based on its row and column
    pub fn compute_square(row: u8, column: u8) -> u8 {
        const WIDTH: u8 = 3;
        let major_row = (row - 1) / WIDTH;
        let major_col = (column - 1) / WIDTH;

        major_col + major_row * WIDTH + 1
    }
}

/// The number(s) marked inside of each cell
#[derive(PartialEq, Eq, Clone, Component)]
pub enum Value {
    /// No value is filled in this cell
    Empty,
    /// A single value is known to be in this cell
    Filled(u8),
    /// We have partial information about the state of this cell
    Marked(CenterMarks, CornerMarks),
}

impl Value {
    /// Converts empty marks into an empty cell state
    fn cleanup(&mut self) -> Value {
        let empty_marks = Value::Marked(CenterMarks::default(), CornerMarks::default());

        if *self == empty_marks {
            return Value::Empty;
        } else {
            return self.clone();
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component, Deref, DerefMut)]
pub struct Cell(pub sudoku::board::Cell);

/// A component that specifies whether digits were provided by the puzzle
#[derive(Component)]
pub struct FixedValue(pub bool);

pub mod marks {
    use bevy::utils::HashSet;

    /// Marks are notes about the possible value of a cell
    pub trait Marks: PartialEq + Eq + Clone {
        /// Creates a new object with only the value entered as its contents
        fn new(num: u8) -> Self;

        /// Updates the value of the marks given a new input
        fn update(&self, num: u8) -> Self;
    }
    /// The value of this cell could be any of the possibilities written in the center of the cell
    #[derive(PartialEq, Eq, Clone, Default)]
    pub struct CenterMarks(HashSet<u8>);

    impl Marks for CenterMarks {
        fn new(num: u8) -> CenterMarks {
            let mut marks = CenterMarks::default();
            marks.0.insert(num);
            marks
        }

        fn update(&self, num: u8) -> CenterMarks {
            let mut out = self.clone();
            if self.0.contains(&num) {
                out.0.remove(&num);
            } else {
                out.0.insert(num);
            }
            out
        }
    }

    impl ToString for CenterMarks {
        fn to_string(&self) -> String {
            let mut vec: Vec<_> = self.0.iter().collect();
            // We want to return the numbers in order, but our storage type is unordered
            vec.sort();
            let maybe_string = vec.iter().map(|m| m.to_string()).reduce(|a, b| a + &b);
            match maybe_string {
                Some(string) => string,
                None => "".to_string(),
            }
        }
    }

    /// The values marked in the corner of this cell must occur in these cells within the square
    #[derive(PartialEq, Eq, Clone, Default)]
    pub struct CornerMarks(HashSet<u8>);

    impl Marks for CornerMarks {
        fn new(num: u8) -> CornerMarks {
            let mut marks = CornerMarks::default();
            marks.0.insert(num);
            marks
        }

        fn update(&self, num: u8) -> CornerMarks {
            let mut out = self.clone();
            if self.0.contains(&num) {
                out.0.remove(&num);
            } else {
                out.0.insert(num);
            }
            out
        }
    }

    impl ToString for CornerMarks {
        fn to_string(&self) -> String {
            let mut vec: Vec<_> = self.0.iter().collect();
            // We want to return the numbers in order, but our storage type is unordered
            vec.sort();
            let maybe_string = vec.iter().map(|m| m.to_string()).reduce(|a, b| a + &b);
            match maybe_string {
                Some(string) => string,
                None => "".to_string(),
            }
        }
    }
}
