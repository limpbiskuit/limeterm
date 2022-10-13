use std::{os::unix::prelude::RawFd, process::Command};

use nix::{pty::forkpty, errno::Errno, unistd::{ForkResult, read}};

use crate::{rendering::{window::{SdlWindow, SdlWindowBuilder, TermWindow}, text::Line}, escaper::Escaper};

pub struct Term {
    pub rows: u16,
    pub cols: u16,

    pub lines: Vec<Line>,

    pub sdl_window: SdlWindow,

    pub window: TermWindow,

    pub pty: RawFd,

    pub escaper: Escaper,
}

impl Term {
    pub fn new(rows: u16, cols: u16, shell: &str, title: &str) -> Self {
        let cw: u32;
        let ch: u32;

        {
            let ttf = sdl2::ttf::init().unwrap();
            let font = ttf.load_font("/usr/share/fonts/noto/NotoSansMono-Regular.ttf", 14).unwrap();
            (cw, ch) = font.size_of_char('I').unwrap();
        }

        let w = cw*cols as u32;
        let h = ch*rows as u32;
        
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

        let pty = spawn_pty(shell).expect("couldn't open shell.");

        let escaper = Escaper::new(rows, cols);

        Self { rows, cols, lines: Vec::new(), sdl_window, window, pty, escaper }
    }

    pub fn read_pty(&self) -> Option<Vec<u8>> {
        let mut read_buffer = [0; 65536];
        let read_result = read(self.pty, &mut read_buffer);
        match read_result {
            Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
            Err(_e) => None,
        }
    }

    pub fn draw_region(&mut self) {
        // let y1: usize = max(self.lines.len() as isize-self.rows as isize, 0) as usize;
        
        for y in 0..self.lines.len() {
            self.lines[y].draw(y, &self.window, &mut self.sdl_window).unwrap();
        }
    }

    pub fn run(&mut self) -> Result<(), String>{
        // self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // self.sdl_window.canvas.clear();
        
        // self.sdl_window.canvas.present();
        
        let mut event_pump = self.sdl_window.context.event_pump()?;

        // dbg!(String::from_utf8(self.read_pty().unwrap()).unwrap());
            

        'running: loop {
            self.sdl_window.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.sdl_window.canvas.clear();

            let mut read_buffer = vec![];

            loop {
                match self.read_pty() {
                    Some(mut bytes) => { read_buffer.append(&mut bytes) }
                    None => { break; }
                }
            }

            let lines = self.escaper.escape(read_buffer);

            self.lines = lines;
            
            self.draw_region();

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => {
                        break 'running
                    },
                    sdl2::event::Event::KeyDown { keycode: Some(kc), .. } => { dbg!(kc); }
                    _ => {}
                }
            }
    
            self.sdl_window.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

fn spawn_pty(default_shell: &str) -> Result<RawFd, Errno> {
    unsafe {
        match forkpty(None, None) {
            Ok(res) => {
                let stdout_fd = res.master; // primary
                if let ForkResult::Child = res.fork_result {
                    // I'm the secondary part of the pty
                    Command::new(&default_shell)
                        .spawn()
                        .expect("failed to spawn pty");
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    std::process::exit(0);
                }
                Ok(stdout_fd)
            },
            Err(e) => Err(e)
        }
    }
}