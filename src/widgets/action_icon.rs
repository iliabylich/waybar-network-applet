use gtk4::{
    prelude::{EventControllerExt, GestureExt, ToVariant, WidgetExt},
    GestureClick, Image,
};

pub(crate) struct ActionIcon {
    icon_name: Option<&'static str>,
    action: Option<&'static str>,
    row_idx: Option<usize>,
}

impl ActionIcon {
    pub(crate) fn new() -> Self {
        Self {
            icon_name: None,
            action: None,
            row_idx: None,
        }
    }

    pub(crate) fn icon_name(mut self, icon_name: &'static str) -> Self {
        self.icon_name = Some(icon_name);
        self
    }

    pub(crate) fn action(mut self, action: &'static str) -> Self {
        self.action = Some(action);
        self
    }

    pub(crate) fn row_idx(mut self, row_idx: usize) -> Self {
        self.row_idx = Some(row_idx);
        self
    }

    pub(crate) fn build(self) -> Image {
        let icon_name = self.icon_name.expect("No icon name provided");
        let row_idx = self.row_idx.expect("No row idx provided");
        let action = self.action.expect("No action provided");

        let icon = Image::builder()
            .icon_name(icon_name)
            .icon_size(gtk4::IconSize::Large)
            .build();

        let gesture = GestureClick::new();

        gesture.connect_released(move |gesture, _, _, _| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            gesture
                .widget()
                .unwrap()
                .activate_action(action, Some(&row_idx.to_string().to_variant()))
                .unwrap();
        });
        icon.add_controller(gesture);
        icon
    }
}
