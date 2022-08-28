use sdl2::{render::{WindowCanvas, BlendMode}, ttf::Font, rect::Rect, pixels::Color};


#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub c: char,
    pub fgcolor: Color,
    pub bgcolor: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub inverted: bool
}

impl Cell {
    pub fn new(c: char, fgcolor: Color, bgcolor: Color) -> Self {
        Cell {
            c,
            fgcolor,
            bgcolor,
            bold: false,
            italic: false,
            underline: false,
            inverted: false
        }
    }
}

pub struct Screen {
    width: usize,
    height: usize,
    cells: Vec<Cell>,

    font_width: u32,
    font_height: u32,

    curr_fgcolor: Color,
    curr_bgcolor: Color,
}

impl Screen {
    pub fn new(width: usize, height: usize, font_width: u32, font_height: u32) -> Self {
        let mut s = Screen { width, height, cells: Vec::new(), font_width, font_height, curr_fgcolor: Color::RGB(255, 255, 255), curr_bgcolor: Color::RGB(55, 55, 55) };
        for _ in 0..width*height {
            s.cells.push(Cell::new(' ', Color::RGB(0, 0, 0), Color::RGB(0, 0, 0)));
        }

        s
    }

    pub fn set_cell(&mut self, cell: Cell, x: u32, y: u32) {
        let i = (x+y*(self.width as u32)) as usize;

        self.cells[i] = cell;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, renderer: &Renderer) {

        let texture_creator = canvas.texture_creator();

        for i in 0..(self.width*self.height) {
            let y = i/self.width;
            let x = i%self.width;

            let cell = self.cells[i];
            let text_surface = renderer.font.render_char(self.cells[i].c)
                .shaded(cell.fgcolor, cell.bgcolor)
                .unwrap();
                
            let mut text_texture = text_surface.as_texture(&texture_creator).unwrap();
            text_texture.set_blend_mode(BlendMode::Add);
            let real_rect = Rect::new((x * ((self.font_width) as usize)) as i32, (y * (self.font_height as usize)) as i32, self.font_width, self.font_height);
            canvas.copy(&text_texture, None, real_rect).unwrap();
        }
    }
}

pub struct Renderer<'a> {
    font: Font<'a, 'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(font: Font<'a, 'a>) -> Self {
        Renderer { font }
    }
}