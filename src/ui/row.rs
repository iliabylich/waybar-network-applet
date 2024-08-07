use crate::Iface;
use gtk4::{
    prelude::BoxExt, Align, BaselinePosition, CenterBox, IconSize, Image, Label, Orientation,
};

pub(crate) struct Row {
    pub(crate) label: Label,
    pub(crate) icon: Image,
}

impl Row {
    pub(crate) fn new_settings(parent: &gtk4::Box) -> Self {
        Self::new(
            "Settings (nmtui)",
            "preferences-system-network",
            "settings-row",
            parent,
        )
    }

    pub(crate) fn new_exit(parent: &gtk4::Box) -> Self {
        Self::new("Close", "window-close", "exit-row", parent)
    }

    fn new(text: impl AsRef<str>, icon_name: &str, css_class: &str, parent: &gtk4::Box) -> Self {
        let label = Label::new(Some(text.as_ref()));

        let icon = Image::builder()
            .icon_name(icon_name)
            .icon_size(IconSize::Large)
            .build();

        let icons_box = gtk4::Box::builder()
            .orientation(Orientation::Horizontal)
            .baseline_position(BaselinePosition::Center)
            .halign(Align::Start)
            .css_classes(["icons"])
            .build();
        icons_box.append(&icon);

        let container = CenterBox::builder()
            .orientation(Orientation::Horizontal)
            .baseline_position(BaselinePosition::Center)
            .halign(Align::Fill)
            .start_widget(&label)
            .end_widget(&icons_box)
            .css_classes(["row", css_class])
            .build();

        parent.append(&container);

        Self { label, icon }
    }
}

pub(crate) struct InterfaceRow(pub(crate) Iface, pub(crate) Row);

impl InterfaceRow {
    pub(crate) fn new_iface(iface: Iface, parent: &gtk4::Box) -> Self {
        let row = Row::new(format!("{}", iface), "edit-copy", "ip-row", parent);

        Self(iface, row)
    }
}
