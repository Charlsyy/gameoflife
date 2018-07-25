mod header;
use self::header::Header;
use gtk;
use gtk::*;
use gtk::traits::*;
use gdk;

use gdk::prelude::*;
//use gdk::{EventMask, ModifierType};

use std::process;

pub const BUTTON_WIDTH: i32 = 1;
pub struct App {
    pub window: Window,
    pub header: Header,
}
impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            println!("failed to init GTK");
            process::exit(1);
        }
        let mut buttons = vec![vec![Button::new(); 10]; 10];

        let window = Window::new(WindowType::Toplevel);
        window.set_hexpand(true);
        let header = Header::new();

        let css_provider = gtk::CssProvider::new();
        let display = gdk::Display::get_default().expect("Couldn't open default GDK display");
        let screen = display.get_default_screen();
        StyleContext::add_provider_for_screen(
            &screen,
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        css_provider
            .load_from_path("src/components/styles/app.css")
            .expect("Failed to load CSS stylesheet");

        let main_container = gtk::Grid::new();
        main_container
            .get_style_context()
            .map(|c| c.add_class("main_container"));
        window.add(&main_container);
        for i in 0..10 {
            for k in 0..10 {
                buttons[i][k] = Button::new();
                buttons[i][k].set_hexpand(true);
                buttons[i][k].set_vexpand(true);
            }
        }

        // let button_clone = buttons[0][0].clone();

        // buttons[0][0].connect_clicked(move |_| {
        //     button_clone.get_style_context().map(|c| c.add_class("black-btn"));
        // });

        for i in 0..10 {
            for k in 0..10 {
                main_container.attach(&buttons[i][k], (i as i32), ((k) as i32), 1, 1);
            }
        }

        window.set_default_size(600, 350);
        window.set_titlebar(&header.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header }
    }
}
