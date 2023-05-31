use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, TreeView, TreeViewColumn, TreeIter, ListStore};

use::std::env::args;
use::std::rc::Rc;
use::std::cell::RefCell;
use::std::collections::HashMap;

fn main() {
    let application = Application::new(
        Some("com.excample.jadwal"),
        Default::default(),
    );
    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Jadwal");
    window.set_default_size(400, 300);

    let main_box = Box::new(Orientation::Vertical, 5);
    window.add(&main_box);

    let tree_view = TreeView::new();
    let collumns = ["Tanggal", "Acara"];
    let list_score = Rc::new(RefCell::new(ListStore::new(&[String::static_type(), String::static_type()])));

    for (date, event) in SCHEDULE.iter() {
        let values: [&dyn ToValue; 2] = [&date.to_string(), &event.to_string()];
        list_store.borrow_mut().insert_with_values(None, &[0, 1], &values);
    }
    
    

    for (i, title) in collumns.iter().enumerate() {
        let column = TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
    
        column.set_title(title);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", i as i32);
    
        tree_view.append_column(&column);
    }
    

    main_box.pack_start(&tree_view, true, true, 0);

    let date_entry = Entry::new();
    main_box.pack_start(&date_entry, false, false, 0);

    let event_entry = Entry::new();
    main_box.pack_start(&event_entry, false, false, 0);

    let add_button = Button::new_with_label("Tambah");
    main_box.pack_start(&add_button, false, false, 0);

    add_button.connect_clicked(move |_| {
        let date = date_entry.get_text().unwrap().to_string();
        let event = event_entry.get_text().unwrap().to_string();

        let values: [&dyn ToValue; 2] = [&date, &event];
        list_score.borrow_mut().insert_with_values(None, &[0, 1], &[&date, &event]);

        SCHEDULE.insert(date, event);

        date_entry.set_text("");
        event_entry.set_text("");
    });

    let remove_button = Button::new_with_label("Hapus");
    main_box.pack_start(&remove_button, false, false, 0);

let list_score_clone = list_score.clone();
remove_button.connect_clicked(move |_| {
    if let Some(selection) = tree_view.selection().selected_rows() {
        if let Some((list_store, iter)) = selection.get_selected() {
            if let Ok(Some(date_value)) = list_store.get_value(&iter, 0).get::<String>() {
                let date = date_value.as_str();
                list_store.remove(&iter);
                SCHEDULE.remove(date);
            }
        }
    }
});

          
    
    let exit_button = Button::with_label("Keluar");
    main_box.pack_start(&exit_button, false, false, 0);
    
    exit_button.connect_clicked(|_| {
        gtk::main_quit();
    });
    
    window.show_all();
    
}

lazy_static::lazy_static! {
    static ref SCHEDULE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("2020-12-25".to_string(), "Natal".to_string());
        m.insert("2021-01-01".to_string(), "Tahun Baru".to_string());
        m
    };
}
