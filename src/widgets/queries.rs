use gtk4::{
    prelude::{Cast, GtkWindowExt, WidgetExt},
    ApplicationWindow, CenterBox, Label,
};

fn get_row_by_idx(window: &ApplicationWindow, idx: usize) -> Option<CenterBox> {
    let list = window.child()?.dynamic_cast::<gtk4::Box>().ok()?;
    let mut row = list.first_child().unwrap();
    for _ in 0..idx {
        row = row.next_sibling().unwrap();
    }
    row.dynamic_cast::<CenterBox>().ok()
}

pub(crate) fn get_label_by_idx(window: &ApplicationWindow, idx: usize) -> Option<Label> {
    let row = get_row_by_idx(window, idx)?;
    row.start_widget()?.dynamic_cast::<Label>().ok()
}
