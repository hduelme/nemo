use gtk::gio::ApplicationFlags;
use gtk::{AccelGroup, Application, CheckMenuItem, IconSize, Image, Label, Menu, MenuBar, MenuItem};
use gtk::prelude::*;
use crate::nemo_places_sidebar::nemo_places_sidebar_init;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("First GTK+ Clock");
    window.set_border_width(10);
    window.set_default_size(200, 200);

    let label = gtk::Label::new(None);
    label.set_text(&"test");


    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);
    let menu_bar = MenuBar::new();
    let file = MenuItem::with_label("File");
    let about = MenuItem::with_label("About");
    let quit = MenuItem::with_label("Quit");
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let file_label = Label::new(Some("File"));
    let folder_item = MenuItem::new();
    let folder_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let folder_image = Image::from_icon_name(Some("folder-music-symbolic"), IconSize::Menu);
    let folder_label = Label::new(Some("Folder"));
    let check_item = CheckMenuItem::with_label("Click me!");

    file_box.pack_start(&file_label, true, true, 0);
    file_item.add(&file_box);
    folder_box.pack_start(&folder_image, false, false, 0);
    folder_box.pack_start(&folder_label, true, true, 0);
    folder_item.add(&folder_box);
    menu.append(&file_item);
    menu.append(&folder_item);
    menu.append(&check_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    let other_menu = Menu::new();
    let sub_other_menu = Menu::new();
    let other = MenuItem::with_label("Another");
    let sub_other = MenuItem::with_label("Sub another");
    let sub_other2 = MenuItem::with_label("Sub another 2");
    let sub_sub_other2 = MenuItem::with_label("Sub sub another 2");
    let sub_sub_other2_2 = MenuItem::with_label("Sub sub another2 2");

    sub_other_menu.append(&sub_sub_other2);
    sub_other_menu.append(&sub_sub_other2_2);
    sub_other2.set_submenu(Some(&sub_other_menu));
    other_menu.append(&sub_other);
    other_menu.append(&sub_other2);
    other.set_submenu(Some(&other_menu));
    menu_bar.append(&other);

    // v_box.pack_start(&menu_bar, false, false, 0);
    // v_box.pack_start(&label, true, true, 0);
    v_box.pack_start(&nemo_places_sidebar_init(), true, true, 0);
    window.add(&v_box);
    window.show_all();
}

pub fn create_new_instance() -> Application {
    let application =
        gtk::Application::new(Some("org.Nemo"), ApplicationFlags::HANDLES_OPEN);
    application.connect_activate(build_ui);
    application.set_register_session(true);
    return application;
}