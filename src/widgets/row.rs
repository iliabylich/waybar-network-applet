use crate::widgets::ActionIcon;

use gtk4::{
    prelude::{BoxExt, WidgetExt},
    CenterBox, Label,
};

pub(crate) struct Row {
    text: Option<String>,
    css_class: Option<String>,
    action_icons: Vec<ActionIcon>,
    idx: Option<usize>,
}

impl Row {
    pub(crate) fn new() -> Self {
        Self {
            text: None,
            css_class: None,
            action_icons: vec![],
            idx: None,
        }
    }

    pub(crate) fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub(crate) fn css_class(mut self, css_class: impl Into<String>) -> Self {
        self.css_class = Some(css_class.into());
        self
    }

    pub(crate) fn action_icon(mut self, icon: ActionIcon) -> Self {
        self.action_icons.push(icon);
        self
    }

    pub(crate) fn idx(mut self, idx: usize) -> Self {
        self.idx = Some(idx);
        self
    }

    pub(crate) fn build(self) -> CenterBox {
        let Self {
            text,
            css_class,
            action_icons: icons,
            idx,
        } = self;
        let text = text.expect("No text provided");
        let idx = idx.expect("No idx provided");

        let label = Label::new(Some(&text));

        let icons_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .baseline_position(gtk4::BaselinePosition::Center)
            .halign(gtk4::Align::Start)
            .css_classes(["icons"])
            .build();
        for icon in icons {
            icons_box.append(&icon.row_idx(idx).build());
        }

        let hbox = CenterBox::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .baseline_position(gtk4::BaselinePosition::Center)
            .halign(gtk4::Align::Fill)
            .start_widget(&label)
            .end_widget(&icons_box)
            .css_classes(["row"])
            .build();

        if let Some(css_class) = css_class {
            hbox.add_css_class(&css_class);
        }

        hbox
    }
}
