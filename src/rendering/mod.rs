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

type GlyphMode = u8;

pub const ATTR_BOLD: u8 = 1 << 0;
pub const ATTR_ITALIC: u8 = 1 << 1;
pub const ATTR_UNDERLINE: u8 = 1 << 2;
pub const ATTR_INVERTED: u8 = 1 << 3;


#[derive(Debug, Clone)]
pub struct Glyph {
    pub c: char,
    pub attributes: GlyphMode,
    pub fgcolor: Color,
    pub bgcolor: Color
}

impl Glyph {
    pub fn new(c: char, fgcolor: Color, bgcolor: Color, bold: bool, italic: bool, underline: bool, inverted: bool) -> Self {
        let mut attributes: GlyphMode = 0;

        if bold { attributes |= ATTR_BOLD }
        if italic { attributes |= ATTR_ITALIC }
        if underline { attributes |= ATTR_UNDERLINE }
        if inverted { attributes |= ATTR_INVERTED }

        Self { c, attributes, fgcolor, bgcolor }
    }
}

impl Glyph {
    pub fn is_bold(&self) -> bool {
        (self.attributes & ATTR_BOLD) == 1
    }

    pub fn is_italic(&self) -> bool {
        (self.attributes & ATTR_ITALIC) == 1
    }

    pub fn is_underline(&self) -> bool {
        (self.attributes & ATTR_UNDERLINE) == 1
    }

    pub fn is_inverted(&self) -> bool {
        (self.attributes & ATTR_INVERTED) == 1
    }

    pub fn draw(&self, x: u32, y: u32) {

    }
}

pub type Line = Vec<Glyph>;