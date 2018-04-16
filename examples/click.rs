extern crate psat_gtk;
extern crate psat;

use psat_gtk::gtk;
use gtk::WidgetExt;

fn main() {
    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let mut window = psat_gtk::PsatWindow { window };

    let click_handler = std::sync::Arc::new(|| {
        println!("clicked!");
    });

    let node = psat::h(psat_gtk::BUTTON, psat_gtk::ButtonProps {
        label: "a button".to_owned(),
        on_click: Some(click_handler),
        ..Default::default()
    }, vec![]);

    psat::render(&mut window, &node);

    window.window.show_all();

    window.window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });

    gtk::main();
}
