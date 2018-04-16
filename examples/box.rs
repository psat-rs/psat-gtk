extern crate psat_gtk;
extern crate psat;

use psat_gtk::gtk;
use gtk::{WidgetExt, GtkWindowExt};

fn main() {
    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let mut window = psat_gtk::PsatWindow { window };
    window.window.set_title("psat-gtk demo");

    let node = psat::h(psat_gtk::BOX, psat_gtk::BoxProps {
        orientation: gtk::Orientation::Horizontal,
        spacing: 0
    }, vec![
        psat::h(psat_gtk::BUTTON, psat_gtk::ButtonProps {label: "a button".to_owned(), ..Default::default()}, vec![]),
        psat::h(psat_gtk::BUTTON, psat_gtk::ButtonProps {label: "button 2".to_owned(), ..Default::default()}, vec![])
    ]);

    psat::render(&mut window, &node);

    window.window.show_all();

    window.window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });

    gtk::main();
}
