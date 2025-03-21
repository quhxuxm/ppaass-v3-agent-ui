mod button;
use std::cell::Cell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Orientation};
use gtk::glib::clone;
use gtk::subclass::prelude::{ButtonImpl, ObjectImpl, ObjectSubclass, WidgetImpl};
use crate::button::CustomButton;
const APP_ID: &str = "com.ppaass-v3-agent.ui.windows";


fn build_ui(application: &Application) {
    let button_increase =CustomButton::with_label("Increase");
    let button_decrease =CustomButton::with_label("Decrease");

    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));


    button_increase.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(number.get().to_string().as_str());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[strong]
        number,
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(number.get().to_string().as_str());
        }
    ));

    // Add buttons to `gtk_box`
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

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    // Run the application
    app.run()
}
