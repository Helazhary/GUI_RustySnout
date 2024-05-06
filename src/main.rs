
use gtk::gio::ffi::GOutputVector;
//use rusqlite::{params, Connection as sqlConnection /* , Result*/};
use rusqlite::{Connection, Result};

use gtk::ffi::gtk_grid_remove_row;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation, Entry, CssProvider, StyleContext, Align, Grid};
use gtk::gdk;

use std::sync::{Arc, Mutex};
    
fn main() -> Result<()> {
    let app = Application::new(Some("com.example.RustySnout"), Default::default());
    
    //----------------------------Start of DB----------------------------
    let conn = Connection::open("data.db")?;

    
    let mut stmt = conn.prepare("SELECT process_name, time, block_number FROM App")?;

    // Execute the SELECT query and collect the results into a vector of vectors
    let mut data: Vec<Vec<String>> = Vec::new();
    let rows = stmt.query_map([], |row| {
        Ok(vec![
            row.get::<_, String>(0)?, // process_name
            row.get::<_, i64>(1)?.to_string(), // time (convert to string)
            row.get::<_, i64>(2)?.to_string(), // block_number
        ])
    })?;

    // Process the results and store them in the vector of vectors
    for row in rows {
        data.push(row?);
    }

    // Now data contains all the rows from the App table
   // println!("{:?}", data);

//------------------------------END od DB-----------------------------    

    // let data = vec![
    //     vec![
    //         "connection".to_string(),
    //         "ID".to_string(),
    //         "Name".to_string(),
    //         "Size".to_string(),
    //         "Extra".to_string(),
    //     ],
    //     vec![
    //         "c1".to_string(),
    //         "0".to_string(),
    //         "goog".to_string(),
    //         "20".to_string(),
    //         "lo".to_string(),
    //     ],
    //     vec![
    //         "c2".to_string(),
    //         "1".to_string(),
    //         "firefox".to_string(),
    //         "30".to_string(),
    //         "lo".to_string(),
    //     ],
    //     vec![
    //         "c3".to_string(),
    //         "2".to_string(),
    //         "amazon".to_string(),
    //         "50".to_string(),
    //         "lo".to_string(),
    //     ],
    // ];

    app.connect_activate( move |app| {
       let shared_content = Arc::new(Mutex::new(data.clone()));
        
        build_ui(app, shared_content.clone());
        
    });

     app.run();

Ok(())

}
//process_data:Vec<&[&str]>
fn build_ui(app: &Application, shared_content: Arc<Mutex<Vec<Vec<String>>>>) {
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
    
    let label = Label::builder()
    .build();
    content.append(&label);

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
        // header_label.set_text(&format!(" Refresh Rate: {}s", process_data));
        setup_window.hide();
        main_window.show();
    });

    // Connect button1
    button1.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        let label=label.clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
            {
                let content = shared_content.lock().unwrap().clone();
                        
           // Iterate over the data and populate the grid
            for (row_index, row) in content.iter().enumerate() {
                for (col_index, value) in row.iter().enumerate() {
                    let label = Label::new(Some(value));
                    data_grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
                }
            }

        
              
                
            }
    }
    });

    // Connect button2
    // button2.connect_clicked({
    //     let display_label = display_label.clone();
    //     let data_grid = data_grid.clone();
    //     move |_| {
    //         display_label.set_label("Displaying usage data by: Connection:");
    //         populate_data_grid(&data_grid, vec![
    //             &["connection", "ID", "Name", "Size", "Extra"],
    //             &["c1", "0", "goog", "20", "lo"],
    //             &["c2", "1", "firefox", "30", "lo"],
    //             &["c3", "2", "amazon", "50", "lo"],
    //         ]);
    //     }
    // });

//     // Connect button3
//     button3.connect_clicked({
//         let display_label = display_label.clone();
//         let data_grid = data_grid.clone();
//         move |_| {
//             display_label.set_label("Displaying usage data by: Remote-Address:");
//             populate_data_grid(&data_grid, vec![
//                 &["remote", "ID", "Name"],
//                 &["r1", "0", "goog"],
//                 &["r2", "1", "firefox"],
//                 &["r3", "2", "amazon"],
//             ]);

//         }
//     });
// }









// fn populate_data_grid(label: &Label, shared_content: Arc<Mutex<String>>) {
//     // Clear previous contents of the data grid
//     //     for j in 0..20{
//     //     grid.remove_row(j);
//     //     grid.remove_column(j)
//     // }

//     // Iterate over the data and populate the grid
//     // for (row_index, row) in data.iter().enumerate() {
//     //     for (col_index, &value) in row.iter().enumerate() {
//     //         let label = Label::new(Some(value));
//     //         grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
//     //     }
//     // }

//     // Set initial text of the label to the shared content
//     {
//         let content = shared_content.lock().unwrap().clone();
//         label.set_text(&content)
//     }
// }

}

