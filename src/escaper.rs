use vt100::Parser;

use crate::rendering::text::{Glyph, Line};


pub struct Escaper {
    pub parser: Parser,
}

impl Escaper {
    pub fn new(rows: u16, cols: u16) -> Self {
        let parser = vt100::Parser::new(rows, cols, 0);
        Self { parser }
    }

    pub fn escape(&mut self, bytes: Vec<u8>) -> Vec<Line> {
        self.parser.process(bytes.as_slice());

        let mut lines: Vec<Line> = vec![];
        for row in 0..self.parser.screen().size().0 {
            let mut glyphs: Vec<Glyph> = vec![];
            for col in 0..self.parser.screen().size().1 {
                let cell = self.parser.screen().cell(row, col).unwrap();

                if cell.contents().len() == 1 {
                    let g = Glyph::new(
                        cell.contents().chars().nth(0).unwrap(),
                        cell.fgcolor().into(), cell.bgcolor().into(),
                        cell.bold(), cell.italic(), cell.underline(), cell.inverse()
                    );
    
                    glyphs.push(g);
                } 
                else if cell.contents().len() > 1 { panic!(); }
                
            }

            let line = Line::new(glyphs);
            lines.push(line);
        }

        lines
    }
    
}