pub extern crate gtk;
extern crate psat;

use gtk::{BoxExt, ButtonExt, ContainerExt, OrientableExt, WidgetExt};
use gtk::Cast;

pub struct PsatWindow {
    pub window: gtk::Window
}

impl psat::Target for PsatWindow {
    type Component = GtkWidget;
    type Context = ();
    fn get_context(&mut self) -> &Self::Context {
        &()
    }
    fn set_root(&mut self, widget: Self::Component) {
        self.window.add(&widget.as_native());
    }
}

struct ChildAccessWrapper<'a, T: 'a> {
    container: &'a mut T,
    children_cache: Vec<GtkWidget>
}

impl<'a, T> std::ops::Deref for ChildAccessWrapper<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<'a, T: gtk::BoxExt + gtk::ContainerExt> psat::ChildAccess<'a, GtkWidget> for ChildAccessWrapper<'a, T> {
    fn len(&self) -> usize {
        self.get_children().len()
    }
    fn insert(&mut self, index: usize, item: GtkWidget) {
        let item = item.as_native();
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
    fn get_mut(&mut self, index: usize) -> Option<&mut GtkWidget> {
        self.children_cache = self.get_children()
            .into_iter()
            .map(|x| GtkWidget::from_native(x))
            .collect();
        if self.children_cache.len() > index {
            Some(&mut self.children_cache[index])
        }
        else {
            None
        }
    }
}

pub enum GtkWidget {
    Button(gtk::Button),
    Box(gtk::Box)
}

impl GtkWidget {
    fn as_native(self) -> gtk::Widget {
        match self {
            GtkWidget::Button(b) => b.upcast(),
            GtkWidget::Box(b) => b.upcast()
        }
    }
    fn from_native(widget: gtk::Widget) -> GtkWidget {
        match widget.downcast::<gtk::Button>() {
            Ok(b) => GtkWidget::Button(b),
            Err(widget) =>
                match widget.downcast::<gtk::Box>() {
                    Ok(b) => GtkWidget::Box(b),
                    Err(_) => panic!("Unexpected component type")
                }
        }
    }
}

pub struct ButtonComponent {}

pub const BUTTON: ButtonComponent = ButtonComponent {};

pub struct ButtonProps {
    pub label: String
}

impl psat::NativeComponent<PsatWindow> for ButtonComponent {
    type Props = ButtonProps;
    fn create(&self, _: &()) -> GtkWidget {
        GtkWidget::Button(gtk::Button::new())
    }
    fn reconcile(&self, _: &(), component: &mut GtkWidget, props: &Self::Props, _: &Vec<psat::VNode<PsatWindow>>) {
        match component {
            &mut GtkWidget::Button(ref mut btn) => {
                btn.set_label(&props.label);
            },
            _ => eprintln!("Component was not a button!")
        }
    }
}

pub struct BoxComponent {}

pub const BOX: BoxComponent = BoxComponent {};

pub struct BoxProps {
    pub orientation: gtk::Orientation,
    pub spacing: i32
}

impl psat::NativeComponent<PsatWindow> for BoxComponent {
    type Props = BoxProps;
    fn create(&self, _: &()) -> GtkWidget {
        GtkWidget::Box(gtk::Box::new(gtk::Orientation::Horizontal, 0))
    }
    fn reconcile(&self,
                 context: &(),
                 component: &mut GtkWidget,
                 props: &Self::Props,
                 children: &Vec<psat::VNode<PsatWindow>>) {
        match component {
            &mut GtkWidget::Box(ref mut b) => {
                b.set_orientation(props.orientation);
                b.set_spacing(props.spacing);
                psat::reconcile_children(context, children, &mut ChildAccessWrapper {container: b, children_cache: vec![]});
            },
            _ => eprintln!("Component was not a box!")
        }
    }
}
