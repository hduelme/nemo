use gtk::prelude::ApplicationExtManual;

pub mod nemo_application;

fn main() {
    let application = nemo_application::create_new_instance();
    application.run();
}