use gtk4::{
    gdk::Display,
    gio::ActionEntry,
    glib::Variant,
    prelude::{DisplayExt, GtkWindowExt, StaticVariantType},
    ApplicationWindow, Label,
};

use crate::{iface_list::IfaceList, widgets::queries};

fn parameter_to_usize(parameter: Option<&Variant>) -> usize {
    parameter
        .unwrap()
        .get::<String>()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

pub(crate) fn copy() -> ActionEntry<ApplicationWindow> {
    ActionEntry::builder("copy-ip")
        .parameter_type(Some(&String::static_variant_type()))
        .activate(|window: &ApplicationWindow, _, parameter| {
            let row_idx = parameter_to_usize(parameter);

            let row = queries::get_label_by_idx(&window, row_idx).unwrap();
            let iface = IfaceList::at(row_idx);

            Display::default()
                .unwrap()
                .clipboard()
                .set_text(&format!("{}", iface.ip));

            row.set_text("Copied!");
            let row_ptr: usize = unsafe { std::mem::transmute(row) };
            gtk4::gio::spawn_blocking(move || {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let row: Label = unsafe { std::mem::transmute(row_ptr) };
                row.set_text(&format!("{}", iface));
            });
        })
        .build()
}

pub(crate) fn open_settings() -> ActionEntry<ApplicationWindow> {
    ActionEntry::builder("open-settings")
        .parameter_type(Some(&String::static_variant_type()))
        .activate(|window: &ApplicationWindow, _, _| {
            std::process::Command::new("kitty")
                .args(["--name", "nmtui", "nmtui"])
                .spawn()
                .unwrap();
            window.close();
        })
        .build()
}

pub(crate) fn close() -> ActionEntry<ApplicationWindow> {
    ActionEntry::builder("close")
        .parameter_type(Some(&String::static_variant_type()))
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build()
}
