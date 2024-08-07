use gtk4::{gdk::Display, CssProvider};

pub(crate) fn load_css() {
    let provider = CssProvider::new();

    provider.connect_parsing_error(|_this, selection, err| {
        eprintln!("Failed to parse CSS:\n{:?}\n{:?}", selection, err)
    });

    let mut dirs = vec![];

    if let Ok(dir) = std::env::current_dir() {
        dirs.push(dir.to_str().unwrap().to_string());
    }

    if let Ok(dir) = std::env::var("HOME") {
        dirs.push(
            std::path::PathBuf::from(dir)
                .join(".config/waybar")
                .to_str()
                .unwrap()
                .to_string(),
        )
    }

    for dir in dirs {
        let path = std::path::PathBuf::from(dir)
            .join("waybar-network-applet.css")
            .to_str()
            .unwrap()
            .to_string();

        load_file(&path, &provider);
    }

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    println!("[CSS] Using styles:\n{}", provider.to_str());
}

fn load_file(path: &str, provider: &CssProvider) {
    if std::fs::read_to_string(path).is_ok() {
        println!("[CSS] Reading {path}...");

        provider.load_from_file(&gtk4::gio::File::for_path(path));
    } else {
        println!("[CSS] Failed to read file {path}");
    }
}
