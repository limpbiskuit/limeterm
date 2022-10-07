pub struct TermWindow {
    pub w: u32,
    pub h: u32,
    pub tw : u32,
    pub th: u32,
    pub cw: u32,
    pub ch: u32
}

pub struct SdlWindowBuilder {
    w: u32,
    h: u32,

    title: String
}

impl SdlWindowBuilder {
    pub fn new() -> Self {
        SdlWindowBuilder { w: 100, h: 100, title: "".to_string() }
    }

    pub fn width(&mut self, w: u32) -> &mut Self {
        self.w = w;

        self
    }

    pub fn heigh(&mut self, h: u32) -> &mut Self {
        self.h = h;

        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();

        self
    }

    pub fn build(&mut self) -> SdlWindow {
        let context = sdl2::init().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let subsystem = context.video().unwrap();
        let window = subsystem.window(self.title.as_str(), self.w, self.h).build().unwrap();

        let canvas = window.into_canvas().build().unwrap();

        SdlWindow { w: self.w, h: self.h, context, ttf_context, canvas }
    }
}

pub struct SdlWindow {
    pub w: u32,
    pub h: u32,

    pub context: sdl2::Sdl,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
    pub canvas: sdl2::render::WindowCanvas
}

impl SdlWindow {
    pub fn run(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        
        self.canvas.present();
        
        let mut event_pump = self.context.event_pump().unwrap();

        'running: loop {
            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }
    
            self.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
