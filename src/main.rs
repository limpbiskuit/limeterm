mod rendering;
mod escaper;

extern crate sdl2;
extern crate nix;
extern crate vt100;

use rendering::{TermScreen, Renderer};
use escaper::Escaper;

use sdl2::pixels::Color;
use sdl2::event::Event;
use vt100::Parser;

use nix::pty::forkpty;
use nix::unistd::{ForkResult, read};

use std::os::unix::prelude::RawFd;
use std::process::Command;
use std::time::Duration;


unsafe fn spawn_pty(default_shell: &str) -> RawFd {
    match forkpty(None, None) {
        Ok(result) => {
            let stdout_fd = result.master; // primary
            if let ForkResult::Child = result.fork_result {
                // I'm the secondary part of the pty
                Command::new(default_shell)
                    .spawn()
                    .expect("failed to spawn pty");
                std::thread::sleep(std::time::Duration::from_millis(2000));
                std::process::exit(0);
            }
            stdout_fd
        },

        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_e) => None,
    }
}

#[allow(dead_code)]
fn get_default_shell() -> String {
    std::env::var("SHELL").expect("Counldn't find default shell")
}

pub fn main() {
    let stdout = unsafe{ spawn_pty("/usr/bin/fish") };

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut renderer = Renderer::new(1200, 800, sdl_context, &ttf_context, "/usr/share/fonts/noto/NotoSansMono-Regular.ttf", 15, "lime terminal emulator");

    let char_surface = renderer.font.render_char('i').shaded(Color::RGB(0, 0, 0), Color::RGB(0, 0, 0)).unwrap();

    let mut screen = TermScreen::new(80, 24, char_surface.width(), char_surface.height());

    let parser = Parser::new(24, 80, 100);

    let mut escaper = Escaper::new(parser);

    renderer.canvas.set_draw_color(Color::RGB(0, 0, 0));
    renderer.canvas.clear();
    renderer.canvas.present();

    let mut event_pump = renderer.sdl_context.event_pump().unwrap();

    escaper.process_str(String::from_utf8(read_from_fd(stdout).unwrap()).unwrap().as_str());
    escaper.set_term_cells(&mut screen);
    // screen.set_cell(Cell::new("i".to_owned(), rendering::Color(255, 255, 255), rendering::Color (0, 0, 0), false, false, false, false), 0, 0);
    println!("{:?}", screen.cells()[0]);

    'running: loop {
        renderer.canvas.set_draw_color(Color::RGB(55, 55, 55));
        renderer.canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        screen.render(&mut renderer.canvas, &renderer.font);

        renderer.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}