use gtk4::{
    gdk::Display,
    glib::clone,
    prelude::{DisplayExt, GestureExt, GtkWindowExt, WidgetExt},
    EventSequenceState, GestureClick,
};

use crate::{
    ui::{InterfaceRow, Row},
    UI,
};

pub(crate) fn add_actions(ui: &UI) {
    let window = &ui.window;

    for InterfaceRow(iface, row) in ui.interface_rows.iter() {
        let label = &row.label;
        let text_to_copy = iface.ip.to_string();

        on_click(
            row,
            clone!(
                #[strong]
                label,
                move || {
                    let original_text = label.text();

                    Display::default()
                        .unwrap()
                        .clipboard()
                        .set_text(&text_to_copy);

                    label.set_text("Copied!");
                    gtk4::glib::spawn_future_local(clone!(
                        #[strong]
                        label,
                        async move {
                            gtk4::glib::timeout_future_seconds(1).await;
                            label.set_text(&original_text);
                        }
                    ));
                }
            ),
        );
    }

    on_click(
        &ui.settings_row,
        clone!(
            #[strong]
            window,
            move || {
                std::process::Command::new("kitty")
                    .args(["--name", "nmtui", "nmtui"])
                    .spawn()
                    .unwrap();
                window.close();
            }
        ),
    );

    on_click(
        &ui.exit_row,
        clone!(
            #[strong]
            window,
            move || {
                window.close();
            }
        ),
    );
}

fn on_click<F>(row: &Row, f: F)
where
    F: 'static + Fn(),
{
    let gesture = GestureClick::new();
    gesture.connect_released(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
        f();
    });
    row.icon.add_controller(gesture);
}
