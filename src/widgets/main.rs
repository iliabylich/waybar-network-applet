use gtk4::{
    prelude::{ActionMapExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

use crate::{actions, tree, widgets::RowList};

pub(crate) struct Main {
    width: i32,
    offset_right: i32,
    row_list: Option<RowList>,
}

impl Main {
    pub(crate) fn new() -> Self {
        Self {
            width: 300,
            offset_right: 200,
            row_list: None,
        }
    }

    pub(crate) fn width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    pub(crate) fn offset_right(mut self, offset_right: i32) -> Self {
        self.offset_right = offset_right;
        self
    }

    pub(crate) fn row_list(mut self, row_list: RowList) -> Self {
        self.row_list = Some(row_list);
        self
    }

    pub(crate) fn build(self, app: &Application) -> ApplicationWindow {
        let Self {
            width,
            offset_right,
            row_list,
        } = self;
        let row_list = row_list.expect("No row list provided");

        let window = ApplicationWindow::builder()
            .application(app)
            .width_request(width)
            .child(&row_list.build())
            .build();

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.auto_exclusive_zone_enable();
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        window.set_margin(Edge::Right, offset_right);

        window.add_action_entries([actions::copy(), actions::open_settings(), actions::close()]);

        window.present();
        tree::print_tree(&window);

        window
    }
}
