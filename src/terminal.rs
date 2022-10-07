use crate::rendering::{Line, window::{SdlWindow, SdlWindowBuilder, TermWindow}};

pub struct Term {
    pub rows: u32,
    pub cols: u32,

    pub lines: Vec<Line>,

    pub sdl_window: SdlWindow,

    pub window: TermWindow,
}

impl Term {
    pub fn new(rows: u32, cols: u32, title: &str) -> Self {
        let sdl_window = SdlWindowBuilder::new()
            .width(800)
            .heigh(600)
            .title(title)
            .build();
            
        let window = TermWindow {
            w: 800, h: 600,
            tw: 800, th: 600,
            cw: 8, ch: 8,
        };

        Self { rows, cols, lines: Vec::<Line>::new(), sdl_window, window }
    }

    pub fn run(&mut self) {
        self.sdl_window.run();
    }
}