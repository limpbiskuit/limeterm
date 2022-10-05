use std::time::Duration;

use sdl2::{Sdl, render::WindowCanvas, pixels::Color, event::Event};

pub struct AppWindow {
    context: Sdl,
    // subsystem: VideoSubsystem,
    canvas: WindowCanvas,
}

impl AppWindow {
    pub fn build() -> Self {
        let context = sdl2::init().unwrap();
        let subsystem = context.video().unwrap();
    
        let window = subsystem.window("limeterm", 1200, 800)
            .resizable()
            .position_centered()
            .build()
            .unwrap();
    
        let canvas = window.into_canvas().build().unwrap();

        Self { context, canvas }
    }

    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        'running: loop {

            let mut event_pump = self.context.event_pump().unwrap();
        
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();
    
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running
                    },
                    _ => {}
                }
            }
    
            self.canvas.present();
            
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    
        }
    }
}