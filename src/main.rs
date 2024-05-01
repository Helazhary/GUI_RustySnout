use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation, Entry, CssProvider, StyleContext, Align};
use gtk::gdk;

fn main() {
    let app = Application::new(Some("com.example.BandwichLog"), Default::default());

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let css = b"* { font-family: monospace; background-color: #1e1e1e; color: #ffffff; } \
                #HeaderLabel { color: #00ff00; } \
                button { margin: 5px; }";

    let provider = CssProvider::new();
    provider.load_from_data(css); // This is correct as it does not return a Result.
    StyleContext::add_provider_for_display(&gdk::Display::default().expect("Error getting default display"),
                                           &provider,
                                           gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let setup_window = ApplicationWindow::builder()
        .application(app)
        .title("Setup - Refresh Rate")
        .default_width(350)
        .default_height(100)
        .build();

    let refresh_rate_entry = Entry::builder()
        .placeholder_text("Enter refresh rate in seconds")
        .build();
    let next_button = Button::builder()
        .label("Next")
        .build();
    let setup_content = Box::new(Orientation::Vertical, 5);
    setup_content.append(&refresh_rate_entry);
    setup_content.append(&next_button);

    setup_window.set_child(Some(&setup_content));
    setup_window.show();

    let main_window = ApplicationWindow::builder()
        .application(app)
        .title("Bandwich Log - Main Window")
        .default_width(1000)
        .default_height(600)
        .build();

    let content = Box::new(Orientation::Vertical, 5);
    let header_label = Label::builder()
        .label("IF: all | Total Rate (Up / Down): 2.22KiB / 4.35KiB")
        .name("HeaderLabel")
        .build();
    let display_label = Label::builder()
        .label("Select a button to display its corresponding data.")
        .build();

    let button1 = Button::builder().label("Firefox").build();
    let button2 = Button::builder().label("Unknown").build();
    let button3 = Button::builder().label("Anydesk").build();

    let buttons_box = Box::new(Orientation::Horizontal, 0);
    buttons_box.set_valign(Align::Center);
    buttons_box.set_halign(Align::Center);
    buttons_box.append(&button1);
    buttons_box.append(&button2);
    buttons_box.append(&button3);

    content.append(&header_label);
    content.append(&display_label);
    content.append(&buttons_box);

    main_window.set_child(Some(&content));

    next_button.connect_clicked(move |_| {
        let refresh_rate = refresh_rate_entry.text().parse::<u64>().unwrap_or(500); // handle errors properly
        header_label.set_text(&format!("IF: all | Refresh Rate: {}s | Total Rate (Up / Down): 2.22KiB / 4.35KiB", refresh_rate));
        setup_window.hide();
        main_window.show();
    });

    button1.connect_clicked({
        let display_label = display_label.clone();
        move |_| {
            display_label.set_label("Process: firefox | Connections: 2 | Rate: 1.87KiB / 4.08KiB");
        }
    });

    button2.connect_clicked({
        let display_label = display_label.clone();
        move |_| {
            display_label.set_label("Process: UNKNOWN | Connections: 3 | Rate: 358.00B / 266.00B");
        }
    });

    button3.connect_clicked({
        let display_label = display_label.clone();
        move |_| {
            display_label.set_label("Process: anydesk | Connections: 1 | Rate: 6.00B / 6.00B");
        }
    });
}
