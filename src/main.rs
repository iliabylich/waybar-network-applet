mod actions;
mod args;
mod css;
mod iface_list;
mod tree;
mod widgets;

use gtk4::{
    gio::ApplicationFlags,
    prelude::{ApplicationExt, ApplicationExtManual},
    Application,
};
use iface_list::IfaceList;
use widgets::{ActionIcon, Main, Row, RowList};

fn iface_rows() -> impl Iterator<Item = Row> {
    IfaceList::get().iter().map(|iface| {
        Row::new()
            .text(format!("{}", iface))
            .css_class("ip-row")
            .action_icon(
                ActionIcon::new()
                    .icon_name("edit-copy")
                    .action("win.copy-ip"),
            )
    })
}

fn settings_row() -> Row {
    Row::new()
        .text("Settings (nmtui)")
        .css_class("settings-row")
        .action_icon(
            ActionIcon::new()
                .icon_name("preferences-system-network")
                .action("win.open-settings"),
        )
}

fn exit_row() -> Row {
    Row::new().text("Close").css_class("exit-row").action_icon(
        ActionIcon::new()
            .icon_name("window-close")
            .action("win.close"),
    )
}

fn draw(app: &Application, args: args::Args) {
    Main::new()
        .width(args.width)
        .offset_right(args.offset_right)
        .row_list(
            RowList::new()
                .rows(iface_rows())
                .row(settings_row())
                .row(exit_row()),
        )
        .build(app);
}

fn main() {
    let args = args::parse_args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    IfaceList::init();

    let app = Application::builder()
        .application_id("com.ilyabylich.waybar_network_applet")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| css::load_css());
    app.connect_activate(move |app| draw(app, args));

    app.run_with_args(&[""]);
}
