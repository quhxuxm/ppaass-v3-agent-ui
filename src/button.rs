use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;


// Object holding the state
#[derive(Default)]
pub struct CustomButtonImpl;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButtonImpl {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButtonImpl {}

// Trait shared by all widgets
impl WidgetImpl for CustomButtonImpl {}

// Trait shared by all buttons
impl ButtonImpl for CustomButtonImpl {}

// Object holding the state
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<CustomButtonImpl>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}