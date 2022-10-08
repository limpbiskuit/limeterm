mod rendering;
mod terminal;

use terminal::Term;

pub fn main() {
    let mut term = Term::new(30, 100, "limeterm");

    term.run().unwrap();
}