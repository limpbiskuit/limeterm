#![allow(non_upper_case_globals)]

use std::ffi::CString;

use x11::xlib::{XRootWindow, XOpenDisplay, XDefaultScreen, XCreateSimpleWindow, XBlackPixel, XWhitePixel, XInternAtom, XSetWMProtocols, XSelectInput, ExposureMask, KeyPressMask, XMapWindow, XNextEvent, XEvent, Expose, XFillRectangle, XDefaultGC, ClientMessage, XDestroyWindow, XCloseDisplay, Atom};

// mod display;
fn main() {
    unsafe {
        let display = XOpenDisplay(std::ptr::null());

        if display.is_null(){
            panic!("Can't open display");
        }

        let screen = XDefaultScreen(display);

        let window = XCreateSimpleWindow(
            display,
            XRootWindow(display, screen),
            10, 10,
            100, 100,
            1,
            XBlackPixel(display, screen),
            XWhitePixel(display, screen),
        );


        let wdelmsg = CString::new("WM_DELETE_WINDOW").unwrap();
        let mut del_window: Atom = XInternAtom(display, wdelmsg.as_ptr(), 0);
        
        XSetWMProtocols(display, window, &mut del_window, 0);
        
        XSelectInput(display, window, ExposureMask | KeyPressMask);

        XMapWindow(display, window);

        let event: *mut XEvent = std::ptr::null_mut();

        'running: loop {
            XNextEvent(display, event);

            match event.as_mut().unwrap().get_type() {
                Expose => {
                    XFillRectangle(display, window, XDefaultGC(display, screen), 20, 20, 10, 10);
                },
                ClientMessage => { break 'running },
                _ => { break 'running; }
            }
        }

        XDestroyWindow(display, window);

        XCloseDisplay(display);
    }
}