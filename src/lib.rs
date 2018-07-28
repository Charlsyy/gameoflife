extern crate gtk;
//  /extern crate gtk_sys;
extern crate gdk;

use gtk::prelude::*;

mod components;
use components::App;

pub fn test_window() {
    let app = App::new();
    app.window.show_all();
    gtk::main();
}
