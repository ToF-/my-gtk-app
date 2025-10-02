use std::rc::Rc;
use std::cell::Cell;
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Orientation};
use crate::glib::clone;

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";

// DOES NOT COMPILE!
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    //
    // A reference counted integer with inner mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(clone!(
            #[weak] number,
            #[weak] button_decrease,
            move |_| {
                number.set(number.get() + 1);
                button_decrease.set_label(&number.get().to_string());
            }));

    button_decrease.connect_clicked(clone!(
            #[strong] number,
            #[weak] button_increase,
            move |_| {
                number.set(number.get() - 1);
                button_increase.set_label(&number.get().to_string());
            }));

    // add buttons to a box
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
