use gtk::prelude::ApplicationExtManual;

pub mod nemo_application;
mod nemo_places_sidebar;

fn main() {
    let application = nemo_application::create_new_instance();
    application.run();
}