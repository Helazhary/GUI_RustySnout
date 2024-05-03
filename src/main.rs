use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation, Entry, CssProvider, StyleContext, Align};
use gtk::gdk;

fn main() {
    let app = Application::new(Some("com.example.RustySnout"), Default::default());

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
        .placeholder_text("Enter refresh rate in seconds (Min: 0.5)")
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
        .title("RustySnout - Main Window")
        .default_width(1000)
        .default_height(600)
        .build();

    let content = Box::new(Orientation::Vertical, 5);
    let header_label = Label::builder()
        .label("--------------------")
        .name("HeaderLabel")
        .build();
    let display_label = Label::builder()
        .label("Select a button to display its corresponding usage data.")
        .build();
    let data_label = Label::builder()
        .label("Data will be displayed here.")
        .build();

    let button1 = Button::builder().label("Process").build();
    let button2 = Button::builder().label("Connection").build();
    let button3 = Button::builder().label("Remote-Address").build();

    let buttons_box = Box::new(Orientation::Horizontal, 0);
    buttons_box.set_valign(Align::Center);
    buttons_box.set_halign(Align::Center);
    buttons_box.append(&button1);
    buttons_box.append(&button2);
    buttons_box.append(&button3);

    content.append(&header_label);
    content.append(&display_label);
    content.append(&buttons_box);
    content.append(&data_label);

    main_window.set_child(Some(&content));

    next_button.connect_clicked(move |_| {
        let mut refresh_rate = refresh_rate_entry.text().parse::<f64>().unwrap_or(0.5); // handle errors properly
        //min refresh rate 0.5
        if (refresh_rate < 0.5) {
            refresh_rate = 0.5;
        }
        header_label.set_text(&format!(" Refresh Rate: {}s", refresh_rate));
        setup_window.hide();
        main_window.show();
    });

    button1.connect_clicked({
        let display_label = display_label.clone();
        let data_label = data_label.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
            data_label.set_label("Process Data will be displayed here.");
        }
    });

    button2.connect_clicked({
        let display_label = display_label.clone();
        let data_label = data_label.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Connection:");
            data_label.set_label("Connection Data will be displayed here.");
        }
    });

    button3.connect_clicked({
        let display_label = display_label.clone();
        let data_label = data_label.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Remote-Address:");
            data_label.set_label("Remote-Address Data will be displayed here.");
        }
    });
}
