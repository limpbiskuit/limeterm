use crate::rendering::{window::{SdlWindow, SdlWindowBuilder, TermWindow}, text::{Line}};

pub struct Term {
    pub rows: u32,
    pub cols: u32,

    pub lines: Vec<Line>,

    pub sdl_window: SdlWindow,

    pub window: TermWindow,
}

impl Term {
    pub fn new(rows: u32, cols: u32, title: &str) -> Self {
        let cw: u32;
        let ch: u32;

        {
            let ttf = sdl2::ttf::init().unwrap();
            let font = ttf.load_font("/usr/share/fonts/noto/NotoSansMono-Regular.ttf", 14).unwrap();
            (cw, ch) = font.size_of_char('I').unwrap();
        }

        let w = cw*cols;
        let h = ch*rows;
        
        let sdl_window = SdlWindowBuilder::new()
            .width(w)
            .heigh(h)
            .title(title)
            .build();

            
        let window = TermWindow {
            w, h,
            tw: 0, th: 0,
            cw, ch,
        };

        Self { rows, cols, lines: Vec::<Line>::new(), sdl_window, window }
    }

    pub fn run(&mut self) -> Result<(), String>{

        let l = Line::new("text");

        self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.sdl_window.canvas.clear();
        
        self.sdl_window.canvas.present();
        
        let mut event_pump = self.sdl_window.context.event_pump()?;

        'running: loop {
            self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.sdl_window.canvas.clear();

            l.draw(1, &self.window, &mut self.sdl_window)?;

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

        Ok(())
    }
}