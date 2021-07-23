use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("xyz.rabuu.steganos")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .title("steganos")
            .build();

        // Show the window.
        window.show();
    });

    app.run();
}
