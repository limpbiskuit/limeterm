use vt100::{Parser, Screen, Cell};

use crate::rendering::TermScreen;

pub struct Escaper {
    pub parser: Parser,
}

impl Escaper {
    pub fn new(parser: Parser) -> Self {
        Self { parser }
    }

    pub fn process_str(&mut self, string: &str) {
        self.parser.process(string.as_bytes());
    }

    pub fn set_term_cells(&self, scr: &mut TermScreen) {
        let w = self.parser.screen().size().1;
        let h = self.parser.screen().size().0;

        for y in 0..h {
            for x in 0..w {
                scr.set_cell(self.parser.screen().cell(y, x).unwrap().clone().into(), x as u32, y as u32);
            }
        }
    }
}