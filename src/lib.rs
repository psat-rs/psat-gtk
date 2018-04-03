pub extern crate gtk;
extern crate psat;

use gtk::{BoxExt, ButtonExt, ContainerExt, OrientableExt, WidgetExt};
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

struct ChildAccessWrapper<'a, T: 'a> {
    container: &'a mut T,
    children_cache: Vec<gtk::Widget>
}

impl<'a, T> std::ops::Deref for ChildAccessWrapper<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<'a, T: gtk::BoxExt + gtk::ContainerExt> psat::ChildAccess<'a, gtk::Widget> for ChildAccessWrapper<'a, T> {
    fn len(&self) -> usize {
        self.get_children().len()
    }
    fn insert(&mut self, index: usize, item: gtk::Widget) {
        self.pack_start(&item, false, false, 0);
        self.reorder_child(&item, index as i32);
    }
    fn relocate(&mut self, b: usize, a: usize) {
        self.reorder_child(&self.get_children()[a], b as i32);
    }
    fn cleanup(&mut self, index: usize) {
        let to_remove = &self.get_children()[index..];
        for widget in to_remove {
            widget.destroy();
        }
    }
    fn get_mut<'b>(&mut self, index: usize) -> Option<&'b mut gtk::Widget> {
        self.children_cache = self.get_children();
        Some(&mut self.children_cache[index])
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

pub struct BoxComponent {}

pub const BOX: BoxComponent = BoxComponent {};

pub struct BoxProps {
    orientation: gtk::Orientation,
    spacing: i32
}

impl psat::NativeComponent<PsatWindow> for BoxComponent {
    type Props = BoxProps;
    fn create(&self, _: &()) -> gtk::Widget {
        gtk::Box::new(gtk::Orientation::Horizontal, 0).upcast()
    }
    fn reconcile(&self,
                 context: &(),
                 component: &mut gtk::Widget,
                 props: &Self::Props,
                 children: &Vec<psat::VNode<PsatWindow>>) {
        modify_as(component, |b: &mut gtk::Box| {
            b.set_orientation(props.orientation);
            b.set_spacing(props.spacing);
            psat::reconcile_children(context, children, &mut ChildAccessWrapper {container: b, children_cache: vec![]});
        });
    }
}
