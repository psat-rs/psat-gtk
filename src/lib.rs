pub extern crate gtk;
extern crate psat;

use gtk::ButtonExt;
use gtk::ContainerExt;
use gtk::Cast;

pub struct PsatWindow {
    pub window: gtk::Window
}

impl psat::Target for PsatWindow {
    type Component = gtk::Widget;
    type Context = ();
    fn get_context(&mut self) -> &Self::Context {
        &()
    }
    fn set_root(&mut self, widget: Self::Component) {
        self.window.add(&widget);
    }
}

// please tell me there's a better way to do this
fn modify_as<T: gtk::IsA<gtk::Widget>, F: Fn(&mut T) -> ()>(widget: &mut gtk::Widget, f: F) {
    let mut casted: T = {
        let widget = widget.to_owned();
        widget.downcast().unwrap()
    };
    f(&mut casted);
}

pub struct ButtonComponent {}

pub const BUTTON: ButtonComponent = ButtonComponent {};

pub struct ButtonProps {
    pub label: String
}

impl psat::NativeComponent<PsatWindow> for ButtonComponent {
    type Props = ButtonProps;
    fn create(&self, _: &()) -> gtk::Widget {
        gtk::Button::new().upcast()
    }
    fn reconcile(&self, _: &(), component: &mut gtk::Widget, props: &Self::Props, _: &Vec<psat::VNode<PsatWindow>>) {
        modify_as(component, |btn: &mut gtk::Button| {
            btn.set_label(&props.label);
        });
    }
}
