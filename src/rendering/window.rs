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

    title: &'static str
}

impl SdlWindowBuilder {
    pub fn new() -> Self {
        SdlWindowBuilder { w: 800, h: 600, title: "basic window" }
    }



    pub fn build(self) -> SdlWindow {
        let context = sdl2::init().unwrap();
        let subsystem = context.video().unwrap();
        let window = subsystem.window(self.title, self.w, self.h).build().unwrap();

        let canvas = window.into_canvas().build().unwrap();

        SdlWindow { w: self.w, h: self.h, context, canvas }
    }
}

pub struct SdlWindow {
    w: u32,
    h: u32,

    context: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas
}

impl SdlWindow {
    pub fn run(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();

        'running: loop {
            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 0));
            self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
    
            self.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}