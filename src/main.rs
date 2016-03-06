extern crate core;
use core::fmt;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

type Position = i32;


fn main() {
    let b : Board = Board::new();
    let x = Mark::X;

    println!("{}", &b);
    println!("{:?}", &b);
    println!("{} {}", x, x.next());
}
