#[allow(deprecated)]
use gtk4::{
    prelude::{StyleContextExt, WidgetExt},
    ApplicationWindow, StyleContextPrintFlags,
};

#[allow(deprecated)]
pub(crate) fn print_tree(window: &ApplicationWindow) {
    // TODO: remove once deprecated, use while available
    println!(
        "[TREE] Widget tree:\n{}",
        window
            .style_context()
            .to_string(StyleContextPrintFlags::RECURSE | StyleContextPrintFlags::SHOW_STYLE)
    );
}
