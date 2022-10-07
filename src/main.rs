use rendering::{Glyph, Color, window::SdlWindowBuilder};

mod rendering;

pub fn main() {
    let mut window = SdlWindowBuilder::new().build();

    window.run();
}