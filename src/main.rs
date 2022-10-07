mod rendering;
mod terminal;

use terminal::Term;

pub fn main() {
    let mut term = Term::new(0, 0, "test");

    term.run();
}