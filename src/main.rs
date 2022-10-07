use gtk::{prelude::*, TextView};
use gtk::{Application, ApplicationWindow};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(460)
            .default_height(69)
            .build();

        let mut view = TextView::builder()
            .margin(6)
            .vexpand(false)
            .build();

        window.add(&view);

        window.show_all();
    });

    application.run();
}