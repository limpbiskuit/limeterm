use sdl2::{rect::Rect, ttf::FontStyle, surface::Surface};

use super::{Color, window::{TermWindow, SdlWindow}};

type GlyphMode = u8;

pub const ATTR_BOLD: u8 = 1 << 0;
pub const ATTR_ITALIC: u8 = 1 << 1;
pub const ATTR_UNDERLINE: u8 = 1 << 2;
pub const ATTR_INVERTED: u8 = 1 << 3;

#[derive(Debug)]
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

    pub fn is_bold(&self) -> bool {
        (self.attributes & ATTR_BOLD) == ATTR_BOLD
    }

    pub fn is_italic(&self) -> bool {
        (self.attributes & ATTR_ITALIC) == ATTR_ITALIC
    }

    pub fn is_underline(&self) -> bool {
        (self.attributes & ATTR_UNDERLINE) == ATTR_UNDERLINE
    }

    pub fn is_inverted(&self) -> bool {
        (self.attributes & ATTR_INVERTED) == ATTR_INVERTED
    }

    pub fn draw(&self, x: usize, y: usize, window: &TermWindow, sdl_window: &mut SdlWindow) -> Result<(), String> {
        let mut font = sdl_window.ttf_context.load_font(sdl_window.font_path.clone(), 14).unwrap();

        if self.is_bold() { font.set_style(FontStyle::BOLD); }
        if self.is_italic() { font.set_style(FontStyle::ITALIC); }
        if self.is_underline() { font.set_style(FontStyle::UNDERLINE); }

        let surface: Surface;
        if !self.is_inverted() {
            surface = font.render_char(self.c).shaded(self.fgcolor, self.bgcolor).unwrap();
        } else {
            surface = font.render_char(self.c).shaded(self.bgcolor, self.fgcolor).unwrap();
        }
        let texture = surface.as_texture(&sdl_window.tex_creator).unwrap();
        let rect = Rect::new((x*window.cw as usize) as i32, (y*window.ch as usize) as i32, window.cw, window.ch);

        sdl_window.canvas.copy(&texture, None, rect)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Line(pub Vec<Glyph>);

impl Line {
    pub fn new(glyphs: Vec<Glyph>) -> Self {
        Self(glyphs)
    }

    pub fn draw(&self, y: usize, window: &TermWindow, sdl_window: &mut SdlWindow) -> Result<(), String> {
        for x in 0..self.0.len() {
            self.0[x].draw(x, y, window, sdl_window)?;
        }
        Ok(())
    }
}