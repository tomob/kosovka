extern crate core;
use core::fmt;

#[derive(Debug)]
enum Mark {
    X,
    O
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum Field {
    Empty,
    M(Mark)
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let f1 : Field = Field::Empty;
    let f2 : Field = Field::M(Mark::X);
    let f3 : Field = Field::M(Mark::O);

    println!("f1: {:?}, f2: {:?}, f3: {:?}", f1, f2, f3);
}
