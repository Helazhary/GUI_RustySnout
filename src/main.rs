
use gtk::builders::ConstraintBuilder;
use gtk::gio::ffi::GOutputVector;

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
    let mut stmt2 = conn.prepare("SELECT pid, process_name, up_bps, down_bps, connections, time, block_number FROM processes")?;
    let mut stmt3 = conn.prepare("SELECT cid, source, destination, protocol, up_bps, down_bps, process_name, time, block_number FROM connections")?;


    let mut data1: Vec<Vec<String>> = Vec::new();
    let mut data2: Vec<Vec<String>> = Vec::new();
    let mut data3: Vec<Vec<String>> = Vec::new();    

    //setting  data 1
    let rows = stmt.query_map([], |row| {
        Ok(vec![
            row.get::<_, String>(0)?, // process_name
            row.get::<_, i64>(1)?.to_string(), // time 
            row.get::<_, i64>(2)?.to_string(), // block_number
        ])
    })?;
    for row in rows {
        data1.push(row?);
    }

    //setting data 2

    let rows2 = stmt2.query_map([], |row| {
        Ok(vec![
            row.get::<_, i64>(0)?.to_string(), // pid
            row.get::<_, String>(1)?, // process_name
            row.get::<_, i64>(2)?.to_string(), // up_bps
            row.get::<_, i64>(3)?.to_string(), // down_bps
            row.get::<_, i64>(4)?.to_string(), // connections
            row.get::<_, i64>(5)?.to_string(), // time
            row.get::<_, i64>(6)?.to_string(), // block_number
        ])
    })?;
    
    for row in rows2 {
        data2.push(row?);
    }

    //setting  data3
    let rows3 = stmt3.query_map([], |row| {
        Ok(vec![
            //row.get::<_, String>(0)?, // cid
            row.get::<_, String>(1)?, // source
            row.get::<_, String>(2)?, // destination
            row.get::<_, String>(3)?, // protocol
            row.get::<_, i64>(4)?.to_string(),// up_bps
            row.get::<_, i64>(5)?.to_string(),// down_bps
            row.get::<_, String>(6)?, //  process_name
            row.get::<_, i64>(7)?.to_string(),  //time
            row.get::<_, i64>(8)?.to_string(),  //block_number
        ])
    })?;

    let mut i=0;
    for row in rows3 {
        // if(i<170){
        //     data3.push(row?);
        // }
        //       i+=1;  

              data3.push(row?);      
    }

//------------------------------END od DB-----------------------------    



    app.connect_activate( move |app| {
       let shared_content1 = Arc::new(Mutex::new(data1.clone()));
       let shared_content2 = Arc::new(Mutex::new(data2.clone()));
       let shared_content3 = Arc::new(Mutex::new(data3.clone()));        
        build_ui(app, shared_content1.clone(),shared_content2.clone(),shared_content3.clone());
        
    });

     app.run();

Ok(())

}
//process_data:Vec<&[&str]>
fn build_ui(app: &Application, shared_content1: Arc<Mutex<Vec<Vec<String>>>>,shared_content2: Arc<Mutex<Vec<Vec<String>>>>,shared_content3: Arc<Mutex<Vec<Vec<String>>>>) {
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
        let content1 = shared_content1.lock().unwrap().clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
            {
                
//            Clear previous contents of the data grid
                for j in 0..20{
                data_grid.remove_row(j);
                data_grid.remove_column(j)
            }                        
           // Iterate over the data and populate the grid
            for (row_index, row) in content1.iter().enumerate() {
                for (col_index, value) in row.iter().enumerate() {
                    let label = Label::new(Some(value));
                    data_grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
                }
            }                

            }
    }
    });

    // Connect button2
     button2.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        let label=label.clone();
        let content2 = shared_content2.lock().unwrap().clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
            {   
//            Clear previous contents of the data grid
                for j in 0..20{
                data_grid.remove_row(j);
                data_grid.remove_column(j)
            }                        
           // Iterate over the data and populate the grid
            for (row_index, row) in content2.iter().enumerate() {
                for (col_index, value) in row.iter().enumerate() {
                    let label = Label::new(Some(value));
                    data_grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
                }
            }                

            }
    }
    });

//     // Connect button3
    button3.connect_clicked({
        let display_label = display_label.clone();
        let data_grid = data_grid.clone();
        let label=label.clone();
        let content3 = shared_content3.lock().unwrap().clone();
        move |_| {
            display_label.set_label("Displaying usage data by: Process:");
            {   
//            Clear previous contents of the data grid
                for j in 0..20{
                data_grid.remove_row(j);
                data_grid.remove_column(j)
            }                        
           // Iterate over the data and populate the grid
            for (row_index, row) in content3.iter().enumerate() {
                for (col_index, value) in row.iter().enumerate() {
                    let label = Label::new(Some(value));
                    data_grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
                }
            }                
            }
    }
    });
}








 //fn populate_data_grid(label: &Label, data_grid: Grid, contetn:Vec<Vec<std::string::String>>) {
//     // Clear previous contents of the data grid
//     //     for j in 0..20{
//     //     grid.remove_row(j);
//     //     grid.remove_column(j)
//     // }

//     // Iterate over the data and populate the grid
    // for (row_index, row) in data.iter().enumerate() {
    //     for (col_index, &value) in row.iter().enumerate() {
    //         let label = Label::new(Some(value));
    //         grid.attach(&label, col_index as i32, row_index as i32, 1, 1);
    //     }
    // }

    // Set initial text of the label to the shared content
    // {
    //     let content = shared_content.lock().unwrap().clone();
    //     label.set_text(&content)
    // }
             
        
// }            
// }


