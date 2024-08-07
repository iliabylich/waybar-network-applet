mod args;
mod css;
mod iface_list;
mod tree;
mod ui;

use gtk4::{
    gio::ApplicationFlags,
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
    Application,
};
use iface_list::Iface;
use ui::UI;

fn main() {
    let args = args::parse_args();

    let app = Application::builder()
        .application_id("com.ilyabylich.waybar_network_applet")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(move |app| {
        let ui = UI::new(app, args);

        ui.window.present();
        tree::print_tree(&ui.window);
    });

    app.connect_startup(|_| css::load_css());

    app.run_with_args(&[""]);
}
