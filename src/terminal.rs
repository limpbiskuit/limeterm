use crate::rendering::{window::{SdlWindow, SdlWindowBuilder, TermWindow}, Color, text::{Line, Glyph}};

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

        let cw: u32;
        let ch: u32;
        {
            let font = sdl_window.ttf_context.load_font("/usr/share/fonts/noto/NotoSansMono-Regular.ttf", 14).unwrap();
            (cw, ch) = font.size_of_char('I').unwrap();
        }
            
        let window = TermWindow {
            w: 800, h: 600,
            tw: 800, th: 600,
            cw, ch,
        };

        Self { rows, cols, lines: Vec::<Line>::new(), sdl_window, window }
    }

    pub fn run(&mut self) {

        let g = Glyph::new('u', Color ( 0, 0, 0 ), Color ( 255, 255, 255 ), false, false, false, false);

        self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.sdl_window.canvas.clear();
        
        self.sdl_window.canvas.present();
        
        let mut event_pump = self.sdl_window.context.event_pump().unwrap();

        'running: loop {
            self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            self.sdl_window.canvas.clear();

            g.draw(0, 0, &self.window, &mut self.sdl_window);

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }
    
            self.sdl_window.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}