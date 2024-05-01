use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation, Entry};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = Application::new(Some("com.example.BandwichLog"), Default::default());
    
    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let window1 = Rc::new(RefCell::new(ApplicationWindow::builder()
        .application(app)
        .title("Bandwich Log - Refresh Rate")
        .default_width(350)
        .default_height(70)
        .build()));

    let window2 = Rc::new(RefCell::new(ApplicationWindow::builder()
        .application(app)
        .title("Bandwich Log - Main Window")
        .default_width(350)
        .default_height(200)
        .build()));

    let content1 = Box::new(Orientation::Vertical, 0);

    let refresh_rate_label = Label::builder()
        .label("Refresh Rate (in seconds):")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let refresh_rate_entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let next_button = Button::builder()
        .label("Next")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    content1.append(&refresh_rate_label);
    content1.append(&refresh_rate_entry);
    content1.append(&next_button);

    window1.borrow().set_child(Some(&content1));
    window1.borrow().show();

    let w1_clone = Rc::clone(&window1);
    let w2_clone = Rc::clone(&window2);

    next_button.connect_clicked(move |_| {
        let refresh_rate_text = refresh_rate_entry.text();
        let refresh_rate = refresh_rate_text.parse::<u64>().unwrap_or(500); // Consider improving this with proper error handling
        w1_clone.borrow().hide();
        create_main_window(&w2_clone, refresh_rate);
    });
}

fn create_main_window(window: &Rc<RefCell<ApplicationWindow>>, refresh_rate: u64) {
    let content = Box::new(Orientation::Vertical, 0);
    let label = Label::builder()
        .label(&format!("Refresh rate set at: {} seconds", refresh_rate))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    content.append(&label);

    window.borrow().set_child(Some(&content));
    window.borrow().show();
}
