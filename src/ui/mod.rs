mod actions;
use actions::add_actions;

mod row;
pub(crate) use row::{InterfaceRow, Row};

use crate::{args::Args, Iface};
use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub(crate) struct UI {
    pub(crate) window: ApplicationWindow,
    pub(crate) interface_rows: Vec<InterfaceRow>,
    pub(crate) settings_row: Row,
    pub(crate) exit_row: Row,
}

impl UI {
    pub(crate) fn new(app: &Application, args: Args) -> Self {
        let window = window(app, args);
        let wrapper = wrapper(&window);
        let interface_rows = Iface::get_all()
            .map(|iface| InterfaceRow::new_iface(iface, &wrapper))
            .collect();
        let settings_row = Row::new_settings(&wrapper);
        let exit_row = Row::new_exit(&wrapper);

        let ui = Self {
            window,
            interface_rows,
            settings_row,
            exit_row,
        };

        add_actions(&ui);

        ui
    }
}

fn window(app: &Application, args: Args) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .width_request(args.width)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.set_margin(Edge::Right, args.offset_right);

    window
}

fn wrapper(window: &ApplicationWindow) -> gtk4::Box {
    let wrapper = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["row-list"])
        .build();

    window.set_child(Some(&wrapper));

    wrapper
}
