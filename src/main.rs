extern crate core;
use core::fmt;

#[derive(Debug, Copy, Clone)]
enum Mark {
    X,
    O
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
enum Field {
    Empty,
    M(Mark)
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
struct Board {
    fields: [Field; 9]
}

impl Board {
    pub fn new() -> Board {
        Board { fields: [Field::Empty; 9]}
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.fields)
    }
}


fn main() {
    let f1 : Field = Field::Empty;
    let f2 : Field = Field::M(Mark::X);
    let f3 : Field = Field::M(Mark::O);

    let b : Board = Board::new();

    println!("f1: {}, f2: {}, f3: {}", f1, f2, f3);
    println!("Board: {}", &b)
}
