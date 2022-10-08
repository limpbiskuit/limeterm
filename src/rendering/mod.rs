pub mod window;
pub mod text;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<vt100::Color> for Color {
    fn from(c: vt100::Color) -> Self {
        match c {

            vt100::Color::Rgb(r, g, b) => {
                Color (r, g, b)
            }

            vt100::Color::Idx(0) => {
                Color (0, 0, 0)
            },
            vt100::Color::Idx(1) => {
                Color (128, 0, 0)
            },
            vt100::Color::Idx(2) => {
                Color (0, 128, 0)
            },
            vt100::Color::Idx(3) => {
                Color (128, 128, 0)
            },
            vt100::Color::Idx(4) => {
                Color (0, 0, 128)
            },
            vt100::Color::Idx(5) => {
                Color (128, 0, 128)
            },
            vt100::Color::Idx(6) => {
                Color (0, 128, 128)
            },
            vt100::Color::Idx(7) => {
                Color (128, 128, 128)
            },
            vt100::Color::Idx(8) => {
                Color (64, 64, 64)
            },
            vt100::Color::Idx(9) => {
                Color (255, 0, 0)
            },
            vt100::Color::Idx(10) => {
                Color (0, 255, 0)
            },
            vt100::Color::Idx(11) => {
                Color (255, 255, 0)
            },
            vt100::Color::Idx(12) => {
                Color (0, 0, 255)
            },
            vt100::Color::Idx(13) => {
                Color (255, 0, 255)
            },
            vt100::Color::Idx(14) => {
                Color (0, 255, 255)
            },
            vt100::Color::Idx(15) => {
                Color (255, 255, 255)
            },

            _ => {
                Color (0, 0, 0)
            }
        }
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color { r: self.0, g: self.1, b: self.2, a: 255 }
    }
}
