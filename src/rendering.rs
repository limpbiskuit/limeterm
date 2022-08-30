use std::hash::BuildHasherDefault;

use sdl2::{render::{WindowCanvas, BlendMode}, ttf::{Font, Sdl2TtfContext}, rect::Rect, Sdl, video::Window, surface::Surface};

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

#[derive(Debug, Clone)]
pub struct Cell {
    pub c: String,
    pub fgcolor: Color,
    pub bgcolor: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverted: bool
}

impl Cell {
    pub fn new(c: String, fgcolor: Color, bgcolor: Color, bold: bool, italic: bool, underline: bool, inverted: bool) -> Self {
        Cell {
            c,
            fgcolor,
            bgcolor,
            bold,
            italic,
            underline,
            inverted
        }
    }
}

impl From<vt100::Cell> for Cell {
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

        Cell::new(c.contents(), fgcolor, bgcolor, c.bold(), c.italic(), c.underline(), c.inverse())
    }
}

pub struct TermScreen {
    width: usize,
    height: usize,
    cells: Vec<Cell>,

    font_width: u32,
    font_height: u32,
}

impl TermScreen {
    pub fn new(width: usize, height: usize, font_width: u32, font_height: u32) -> Self {
        let mut s = TermScreen { width, height, cells: Vec::new(), font_width, font_height };
        for _ in 0..width*height {
            s.cells.push(Cell::new(" ".to_owned(), Color (0, 0, 0), Color (0, 0, 0), false, false, false, false));
        }

        s
    }

    pub fn set_cell(&mut self, cell: Cell, x: u32, y: u32) {
        let i = (x+y*(self.width as u32)) as usize;
        self.cells[i] = cell;
    }

    pub fn set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, font: &Font) {

        let texture_creator = canvas.texture_creator();

        for y in 0..self.height {
            for x in 0..self.width {

                let cell = self.cells[x+(y*self.width)].clone();

                let text_surface: Surface;

                if cell.c != "" {
                    text_surface = font.render(cell.c.as_str())
                    .shaded(cell.fgcolor, cell.bgcolor)
                    .unwrap();
                } else {
                    text_surface = font.render_char(' ')
                    .shaded(cell.fgcolor, cell.bgcolor)
                    .unwrap();
                }
                    
                let mut text_texture = text_surface.as_texture(&texture_creator).unwrap();
                text_texture.set_blend_mode(BlendMode::Add);
                let real_rect = Rect::new((x * ((self.font_width) as usize)) as i32, (y * (self.font_height as usize)) as i32, self.font_width, self.font_height);
                canvas.copy(&text_texture, None, real_rect).unwrap();
            }
        }
    }
}

pub struct Renderer<'a> {
    pub sdl_context: Sdl,
    pub canvas: WindowCanvas,
    pub font: Font<'a, 'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(widht: u32, height: u32, sdl_context: Sdl, ttf_context: &'a Sdl2TtfContext, font_path: &str, font_size: u16, title: &str) -> Self {
        let sdl_window = sdl_context.video().unwrap().window(title, widht, height).build().unwrap();
        let canvas = sdl_window.into_canvas().build().unwrap();
        let font = ttf_context.load_font(font_path, font_size).unwrap();

        Renderer {
            sdl_context,
            canvas,
            font
        }
    }
}