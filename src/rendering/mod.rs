pub mod window;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<vt100::Color> for Color {
    fn from(c: vt100::Color) -> Self {
        match c {

            vt100::Color::Rgb(r, g, b) => {
                Color (r, g, b)
            }

            vt100::Color::Idx(0) => {
                Color (0, 0, 0)
            },
            vt100::Color::Idx(1) => {
                Color (128, 0, 0)
            },
            vt100::Color::Idx(2) => {
                Color (0, 128, 0)
            },
            vt100::Color::Idx(3) => {
                Color (128, 128, 0)
            },
            vt100::Color::Idx(4) => {
                Color (0, 0, 128)
            },
            vt100::Color::Idx(5) => {
                Color (128, 0, 128)
            },
            vt100::Color::Idx(6) => {
                Color (0, 128, 128)
            },
            vt100::Color::Idx(7) => {
                Color (128, 128, 128)
            },
            vt100::Color::Idx(8) => {
                Color (64, 64, 64)
            },
            vt100::Color::Idx(9) => {
                Color (255, 0, 0)
            },
            vt100::Color::Idx(10) => {
                Color (0, 255, 0)
            },
            vt100::Color::Idx(11) => {
                Color (255, 255, 0)
            },
            vt100::Color::Idx(12) => {
                Color (0, 0, 255)
            },
            vt100::Color::Idx(13) => {
                Color (255, 0, 255)
            },
            vt100::Color::Idx(14) => {
                Color (0, 255, 255)
            },
            vt100::Color::Idx(15) => {
                Color (255, 255, 255)
            },

            _ => {
                Color (0, 0, 0)
            }
        }
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color { r: self.0, g: self.1, b: self.2, a: 255 }
    }
}

type GlyphMode = u32;

#[derive(Debug, Clone)]
pub struct Glyph {
    pub c: String,
    pub mode: GlyphMode,
    pub fgcolor: Color,
    pub bgcolor: Color
}

impl Glyph {
    pub fn new(c: String, fgcolor: Color, bgcolor: Color, bold: bool, italic: bool, underline: bool, inverted: bool) -> Self {
        let mut mode: GlyphMode = 0;

        if bold { mode |= 1 << 0 }
        if italic { mode |= 1 << 1 }
        if underline { mode |= 1 << 2 }
        if inverted { mode |= 1 << 3 }

        Self { c, mode, fgcolor, bgcolor }
    }
}

impl From<vt100::Cell> for Glyph {
    fn from(c: vt100::Cell) -> Self {
        let fgcolor;
        let bgcolor;

        match c.fgcolor() {
            vt100::Color::Default => { fgcolor = Color(200, 200, 200); }
            _ => { fgcolor = c.fgcolor().into() }
        }

        match c.bgcolor() {
            vt100::Color::Default => { bgcolor = Color(0, 0, 0); }
            _ => { bgcolor = c.bgcolor().into() }
        }

        Self::new(c.contents(), fgcolor, bgcolor, c.bold(), c.italic(), c.underline(), c.inverse())
    }
}
