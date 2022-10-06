use x11::{xlib::{Display, Colormap, Window, Drawable, GC}, xft::XftColor};

use super::font::Font;

type Color = XftColor;

pub enum WindowMode {}

pub enum CursorMode {}

pub struct TermWindow {
    pub tw: u32,
    pub th: u32,
    pub w: u32,
    pub h: u32,
    pub cw: u32,
    pub ch: u32,
    pub mode: WindowMode,
    pub cursor: CursorMode
}

pub struct XWindow {
    display: Display,
    colormap: Colormap,
    screen: i32,
    window: Window,
    buffer: Drawable,

    fixed: bool,
    left_offset: i32,
    top_offset: i32
}

pub struct DrawingContext {
    color: Color,
    font: Font,
    graphic_context: GC,
}