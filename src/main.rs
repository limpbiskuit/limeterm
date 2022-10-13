mod rendering;
mod escaper;
mod terminal;

use terminal::Term;

pub fn main() {
    let mut term = Term::new(30, 100, "/bin/fish", "limeterm");

    term.run().unwrap();
}