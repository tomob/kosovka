extern crate core;
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mark {X, O}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Mark {
    fn next(&self) -> Mark {
        match *self {
            Mark::X => Mark::O,
            Mark::O => Mark::X
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Field {
    Empty,
    M(Mark)
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Field::Empty => write!(f, " "),
            Field::M(x)  => write!(f, "{}", x)
        }
    }
}

type Line = (Field, Field, Field);
type LinePos = (usize, usize, usize);

const INDICES : [LinePos; 8] = [(0, 1, 2), (3, 4, 5), (6, 7, 8), // horizontal
                                     (0, 3, 6), (1, 4, 7), (2, 5, 8), // vertical
                                     (0, 4, 8), (2, 4, 6)];           // diagonal

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Board {
    fields: [Field; 9]
}

impl Board {
    pub fn new() -> Board {
        Board { fields: [Field::Empty; 9]}
    }

    fn show_line(&self, f: &mut fmt::Formatter, line: usize) -> fmt::Result {
        write!(f, "   {} | {} | {}",
               self.fields[line*3],
               self.fields[line*3 + 1],
               self.fields[line*3 + 2])
    }

    /**
     * Positions:
     * 123
     * 456
     * 789
     */
    pub fn play(&self, mark: Mark, position: Position) -> Board {
        let mut new_fields = self.fields; // Copies the array
        new_fields[position - 1] = Field::M(mark);
        Board { fields: new_fields }
    }

    /// Evaluates a board position:
    /// +1000 is a winning position
    /// -1000 is a loosing position
    ///     0 is a balance between both players
    // pub fn evaluate(&self) -> i32 {
    // }

    fn split_lines(&self) -> Vec<Line> {
        INDICES.iter()
               .map(|x: &LinePos| self.line(*x))
               .collect()
    }

    fn line(&self, line_indices: LinePos) -> Line {
        (self.fields[line_indices.0],
         self.fields[line_indices.1],
         self.fields[line_indices.2])
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "123"));
        try!(self.show_line(f, 0));
        try!(write!(f, "\n456  --- --- ---\n789"));
        try!(self.show_line(f, 1));
        try!(write!(f, "\n     --- --- ---\n   "));
        self.show_line(f, 2)
    }
}

type Position = usize;


fn main() {
    let b : Board = Board::new();
    let x = Mark::X;

    println!("{}", &b);
    println!("{:?}", &b);
    println!("{} {}", x, x.next());
    println!("{}", b.play(Mark::X, 1));
}

#[cfg(test)]
mod tests {
    use super::{Mark, Field, Board, INDICES};

    #[test]
    fn test_next() {
        assert_eq!(Mark::X, Mark::O.next());
        assert_eq!(Mark::O, Mark::X.next());
    }

    #[test]
    fn test_play() {
        let b1 = Board::new();
        let b2 = b1.play(Mark::X, 1);

        assert_eq!(Field::M(Mark::X), b2.fields[0]);
    }

    #[test]
    fn test_line() {
        use Mark::*;
        use Field::*;
        let b = Board::new().play(X, 1).play(O, 9);

        assert_eq!((M(X),  Empty, Empty), b.line(INDICES[0]));
        assert_eq!((Empty, Empty, Empty), b.line(INDICES[1]));
        assert_eq!((Empty, Empty, M(O) ), b.line(INDICES[2]));
        assert_eq!((M(X),  Empty, Empty), b.line(INDICES[3]));
        assert_eq!((Empty, Empty, Empty), b.line(INDICES[4]));
        assert_eq!((Empty, Empty, M(O) ), b.line(INDICES[5]));
        assert_eq!((M(X),  Empty, M(O) ), b.line(INDICES[6]));
        assert_eq!((Empty, Empty, Empty), b.line(INDICES[7]));
    }

    #[test]
    fn split_lines() {
        use Mark::*;
        use Field::*;
        let b = Board::new().play(X, 1).play(O, 9);

        let lines = b.split_lines();
        assert_eq!(8, lines.len());
        assert_eq!((M(X),  Empty, Empty), lines[0]);
        assert_eq!((Empty, Empty, Empty), lines[1]);
        assert_eq!((M(X),  Empty, M(O) ), lines[6]);
    }
}
