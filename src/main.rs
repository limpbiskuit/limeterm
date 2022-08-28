mod rendering;

extern crate sdl2;
extern crate nix;
extern crate vt100;

use rendering::{Screen, Renderer, Cell};
use sdl2::pixels::Color;
use sdl2::event::Event;

// use nix::pty::forkpty;
// use nix::unistd::{ForkResult, read};

// use std::os::unix::prelude::RawFd;
// use std::process::Command;
use std::time::Duration;

// unsafe fn spawn_pty(default_shell: &str) -> RawFd {
//     match forkpty(None, None) {
//         Ok(result) => {
//             let stdout_fd = result.master; // primary
//             if let ForkResult::Child = result.fork_result {
//                 // I'm the secondary part of the pty
//                 Command::new(default_shell)
//                     .spawn()
//                     .expect("failed to spawn pty");
//                 std::thread::sleep(std::time::Duration::from_millis(2000));
//                 std::process::exit(0);
//             }
//             stdout_fd
//         },

//         Err(e) => {
//             panic!("{}", e);
//         }
//     }
// }

// fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
//     let mut read_buffer = [0; 65536];
//     let read_result = read(fd, &mut read_buffer);
//     match read_result {
//         Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
//         Err(_e) => None,
//     }
// }

// fn get_default_shell() -> String {
//     std::env::var("SHELL").expect("Counldn't find default shell")
// }

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let font = ttf_context.load_font("/usr/share/fonts/TTF/FiraCode-Retina.ttf", 15).unwrap();

    let char_surface = font.render_char('i').shaded(Color::RGBA(0, 0, 0, 0), Color::RGBA(0, 0, 0, 0)).unwrap();

    let sdl_window = video_subsystem.window("lime terminal emulator", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = sdl_window.into_canvas().build().unwrap();

    let mut screen = Screen::new(80, 24, char_surface.width(), char_surface.height());
    screen.set_cell(Cell::new('i', Color::RGB(255, 0, 255), Color::RGB(255, 255, 0)), 0, 0);
    screen.set_cell(Cell::new('s', Color::RGB(255, 0, 255), Color::RGB(255, 255, 0)), 1, 0);
    screen.set_cell(Cell::new('s', Color::RGB(255, 0, 255), Color::RGB(0, 0, 255)), 2, 0);
    screen.set_cell(Cell::new('o', Color::RGB(255, 128, 255), Color::RGB(0, 0, 255)), 3, 0);
    screen.set_cell(Cell::new('u', Color::RGB(255, 255, 255), Color::RGB(0, 0, 255)), 4, 0);

    let renderer = Renderer::new(font);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(55, 55, 55));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        screen.render(&mut canvas, &renderer);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}