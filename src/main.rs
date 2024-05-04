use gtk::ffi::gtk_grid_remove_row;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation, Entry, CssProvider, StyleContext, Align, Grid};
use gtk::gdk;

fn main() {
    let app = Application::new(Some("com.example.RustySnout"), Default::default());
    
    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let css=b"/* CSS */
    * { font-family: monospace; background-color: #1e1e1e; color: #ffffff; } \
                #HeaderLabel { color: #00ff00; } \
                button { margin: 5px; }
    grid {
        border: 1px solid #ccc;        
        padding: 10px;
    }
    
";    


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
    let data_grid = Grid::new();
    data_grid.set_row_spacing(10);
    data_grid.set_column_spacing(10);
    data_grid.set_row_homogeneous(true);
    data_grid.set_column_homogeneous(true);
    // data_grid.add_css_class("Grid");
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
    content.append(&data_grid); // Add the data grid here

    main_window.set_child(Some(&content));

    next_button.connect_clicked(move |_| {
        let mut refresh_rate = refresh_rate_entry.text().parse::<f64>().unwrap_or(0.5); // handle errors properly
        //min refresh rate 0.5
        if refresh_rate < 0.5 {
            refresh_rate = 0.5;
        }
        header_label.set_text(&format!(" Refresh Rate: {}s", refresh_rate));
        setup_window.hide();
        main_window.show();
    });

    // Connect button1
    button1.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
                populate_data_grid(&data_grid, vec![
                    &["Process", "ID", "Name", "Size"],
                    &["p1", "0", "goog", "20"],
                    &["p2", "1", "firefox", "30"],
                    &["P3", "2", "amazon", "50"],
            ]);
        }
    });

    // Connect button2
    button2.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Connection:");
            populate_data_grid(&data_grid, vec![
                &["connection", "ID", "Name", "Size", "Extra"],
                &["c1", "0", "goog", "20", "lo"],
                &["c2", "1", "firefox", "30", "lo"],
                &["c3", "2", "amazon", "50", "lo"],
            ]);
        }
    });

    // Connect button3
    button3.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Remote-Address:");
            populate_data_grid(&data_grid, vec![
                &["remote", "ID", "Name"],
                &["r1", "0", "goog"],
                &["r2", "1", "firefox"],
                &["r3", "2", "amazon"],
            ]);

        }
    });
}

// Function to populate the data grid with dummy data
// fn populate_data_grid(data_grid: &Grid, data: Vec<(&str, &str)>) {

//     for j in 0..10{
//         data_grid.remove_row(j);
//     }

//     for (row, (label_text, value_text)) in data.iter().enumerate() {
//         let label_label = Label::new(Some(*label_text));
//         let value_label = Label::new(Some(*value_text));
//         data_grid.attach(&label_label, 0, row as i32, 1, 1);
//         data_grid.attach(&value_label, 1, row as i32, 1, 1);
//     }
// }

fn populate_data_grid(grid: &Grid, data: Vec<&[&str]>) {
    // Clear previous contents of the data grid
        for j in 0..20{
        grid.remove_row(j);
        grid.remove_column(j)
    }

    // Iterate over the data and populate the grid
    for (row_index, row) in data.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            let label = Label::new(Some(value));
            grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
        }
    }

    // Iterate over the data and populate the grid (with adding classes for each row to facilitate styling)
    // for (row_index, row_data) in data.iter().enumerate() {
    //     let row = Box::new(Orientation::Horizontal, 0); // Create a new row box
    //     row.set_css_classes( &["grid-row"]); // Add the grid-row class to the row
    
    //     for (col_index, &value) in row_data.iter().enumerate() {
    //         let label = Label::new(Some(value));
    //         row.append(&label); // Append label to the row
    //     }
    
    //     grid.attach(&row, 0, row_index as i32, 1, 1); // Attach the row to the grid
    // }
}

