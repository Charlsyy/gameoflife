//extern crate gameoflife;
extern crate gdk;
extern crate gtk;
extern crate rustc_serialize;

use rustc_serialize::json::{self, Json, ToJson};
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use std::rc::Rc;

use std::io::prelude::*;
use std::io;
use std::cell::RefCell;

use gtk::*;
use gtk::traits::*;
use gdk::RGBA;

use gdk::prelude::*;

use std::process;
//use gameoflife::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

pub const BUTTON_WIDTH: i32 = 1;
const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[derive(Copy, Clone)]
struct Field {
    active: bool,
    upper_left: bool,
    upper: bool,
    upper_right: bool,

    left: bool,
    right: bool,

    lower_left: bool,
    lower: bool,
    lower_right: bool,
}

struct Header {
    pub container: HeaderBar,
}

struct App {
    pub window: Window,
    pub header: Header,
    pub buttons: Vec<Vec<ToggleButton>>,
}
// 
fn main() {
    #[feature(type_ascription)]
    let mut array_struct = vec![
        vec![
            Field {
                active: false,

                upper_left: false,
                upper: false,
                upper_right: false,

                left: false,
                right: false,

                lower_left: false,
                lower: false,
                lower_right: false,
            };
            10
        ];
        10
    ];
    
    //let mut buffer = File::create(&Path::new("test.json"))?;
    let mut actives = vec![vec![false;HEIGHT];WIDTH];

let mut instant = Instant::now();
let one_sec = Duration::from_secs(1);

    let app = App::new();
    array_struct[2][2].active = true;
    array_struct[2][3].active = true;
    array_struct[3][3].active = true;
    array_struct[1][3].active = true;    
    array_struct[2][4].active = true;
    app.window.show_all();

    let refapp = Rc::new(RefCell::new(app));
    let refapp_copy = refapp.clone();
    let refactives = Rc::new(RefCell::new(actives));
    let refactives_copy = refactives.clone();
    let refarray = Rc::new(RefCell::new(array_struct));
    let refarray_copy = refarray.clone();
    gtk::idle_add(move||{
        if (instant.elapsed() >= one_sec){
            instant+= one_sec;
        update_array(&mut refarray_copy.borrow_mut(), &mut refactives.borrow_mut());
        next_turn(&mut refarray_copy.borrow_mut());
        update_display(&mut refapp_copy.borrow_mut(), &mut refactives_copy.borrow_mut());
        }
        Continue(true)
        
    });
    gtk::main();

    
    

}

impl App {
     fn new() -> App {
        if gtk::init().is_err() {
            println!("failed to init GTK");
            process::exit(1);
        }
        let window = Window::new(WindowType::Toplevel);
        window.set_hexpand(false);
        window.set_vexpand(false);
        window.resize(300, 300);
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

        let mut buttons = vec![vec![ToggleButton::new(); WIDTH]; HEIGHT];
        let main_container = gtk::Grid::new();
        main_container
            .get_style_context()
            .map(|c| c.add_class("main_container"));
        window.add(&main_container);
        for i in 0..WIDTH {
            for k in 0..HEIGHT {
                buttons[i][k] = ToggleButton::new();
                buttons[i][k].set_hexpand(true);
                buttons[i][k].set_vexpand(true);
                buttons[i][k].connect_clicked(move |_| {
        });
                
            }
        }
        for i in 0..WIDTH {
        for k in 1..HEIGHT {
                main_container.attach(&buttons[i][k], (i as i32), ((k) as i32), 1, 1);
                
            }
        }
        //buttons[2][2].set_active(true);
        window.set_default_size(600, 350);
        window.set_titlebar(&header.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // while (1==1){

        // }

        App { window, header, buttons }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();
        container.set_show_close_button(true);
        container.set_title("game of life");
        let hello_btn = Button::new_with_label("Test!");
        let other_button = Button::new_with_label("Test!");
        container.pack_start(&hello_btn);
        container.pack_start(&other_button);
        hello_btn
            .get_style_context()
            .map(|c| c.add_class("hello-btn"));
        other_button
            .get_style_context()
            .map(|c| c.add_class("hello-btn"));

        Header { container }
    }
}



fn update_display(app: &mut App, actives: &mut Vec<Vec<bool>>) {
        for i in 0..WIDTH {
            for k in 0..HEIGHT {
                app.buttons[i][k].set_active(actives[i][k]);
            }
        }
    }

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn update_array(array_struct: &mut Vec<Vec<Field>>, actives: &mut Vec<Vec<bool>>) {
    for i in 1..HEIGHT - 1 {
        for k in 1..WIDTH - 1 {
            array_struct[i][k].upper_left = array_struct[i - 1][k - 1].active;
            array_struct[i][k].upper = array_struct[i - 1][k].active;
            array_struct[i][k].upper_right = array_struct[i - 1][k + 1].active;

            array_struct[i][k].left = array_struct[i][k - 1].active;
            array_struct[i][k].right = array_struct[i][k + 1].active;

            array_struct[i][k].lower_left = array_struct[i + 1][k - 1].active;
            array_struct[i][k].lower = array_struct[i + 1][k].active;
            array_struct[i][k].lower_right = array_struct[i + 1][k + 1].active;
        }
    }

    for i in 1..HEIGHT - 1 {
        array_struct[i][0].upper_left = array_struct[i - 1][WIDTH - 1].active;
        array_struct[i][0].left = array_struct[i][WIDTH - 1].active;
        array_struct[i][0].lower_left = array_struct[i + 1][WIDTH - 1].active;

        array_struct[i][0].upper = array_struct[i - 1][0].active;
        array_struct[i][0].upper_right = array_struct[i - 1][1].active;
        array_struct[i][0].right = array_struct[i][1].active;

        array_struct[i][0].lower = array_struct[i + 1][0].active;
        array_struct[i][0].lower_right = array_struct[i + 1][1].active;
    }

    for i in 1..HEIGHT - 1 {
        array_struct[i][WIDTH - 1].upper_right = array_struct[i - 1][0].active;
        array_struct[i][WIDTH - 1].right = array_struct[i][0].active;
        array_struct[i][WIDTH - 1].lower_right = array_struct[i + 1][0].active;
        array_struct[i][WIDTH - 1].upper_left = array_struct[i - 1][WIDTH - 2].active;
        array_struct[i][WIDTH - 1].upper = array_struct[i - 1][WIDTH - 1].active;

        array_struct[i][WIDTH - 1].left = array_struct[i][WIDTH - 2].active;

        array_struct[i][WIDTH - 1].lower_left = array_struct[i + 1][WIDTH - 2].active;
        array_struct[i][WIDTH - 1].lower = array_struct[i + 1][WIDTH - 1].active;
    }

    for i in 1..WIDTH - 1 {
        array_struct[0][i].upper_left = array_struct[HEIGHT - 1][i - 1].active;
        array_struct[0][i].upper = array_struct[HEIGHT - 1][i].active;
        array_struct[0][i].upper_right = array_struct[HEIGHT - 1][i + 1].active;

        array_struct[0][i].left = array_struct[0][i - 1].active;
        array_struct[0][i].right = array_struct[0][i + 1].active;

        array_struct[0][i].lower_left = array_struct[1][i - 1].active;
        array_struct[0][i].lower = array_struct[1][i].active;
        array_struct[0][i].lower_right = array_struct[1][i + 1].active;
    }

    for i in 1..WIDTH - 1 {
        array_struct[HEIGHT - 1][i].lower_left = array_struct[0][i - 1].active;
        array_struct[HEIGHT - 1][i].lower = array_struct[0][i].active;
        array_struct[HEIGHT - 1][i].lower_right = array_struct[0][i + 1].active;

        array_struct[HEIGHT - 1][i].upper_left = array_struct[HEIGHT - 2][i - 1].active;
        array_struct[HEIGHT - 1][i].upper = array_struct[HEIGHT - 2][i].active;
        array_struct[HEIGHT - 1][i].upper_right = array_struct[HEIGHT - 2][i + 1].active;

        array_struct[HEIGHT - 1][i].left = array_struct[HEIGHT - 1][i - 1].active;
        array_struct[HEIGHT - 1][i].right = array_struct[HEIGHT - 1][i + 1].active;
    }

    array_struct[0][0].upper_left = array_struct[HEIGHT - 1][WIDTH - 1].active;
    array_struct[0][0].upper = array_struct[HEIGHT - 1][0].active;
    array_struct[0][0].upper_right = array_struct[HEIGHT - 1][1].active;

    array_struct[0][0].left = array_struct[0][WIDTH - 1].active;
    array_struct[0][0].right = array_struct[0][1].active;

    array_struct[0][0].lower_left = array_struct[1][WIDTH - 1].active;
    array_struct[0][0].lower = array_struct[1][0].active;
    array_struct[0][0].lower_right = array_struct[1][1].active;

    array_struct[0][WIDTH - 1].upper_left = array_struct[HEIGHT - 1][WIDTH - 2].active;
    array_struct[0][WIDTH - 1].upper = array_struct[HEIGHT - 1][WIDTH - 1].active;
    array_struct[0][WIDTH - 1].upper_right = array_struct[HEIGHT - 1][0].active;

    array_struct[0][WIDTH - 1].left = array_struct[0][WIDTH - 2].active;
    array_struct[0][WIDTH - 1].right = array_struct[0][0].active;

    array_struct[0][WIDTH - 1].lower_left = array_struct[1][WIDTH - 2].active;
    array_struct[0][WIDTH - 1].lower = array_struct[1][WIDTH - 1].active;
    array_struct[0][WIDTH - 1].lower_right = array_struct[1][0].active;

    array_struct[HEIGHT - 1][0].upper_left = array_struct[HEIGHT - 2][WIDTH - 1].active;
    array_struct[HEIGHT - 1][0].upper = array_struct[HEIGHT - 2][0].active;
    array_struct[HEIGHT - 1][0].upper_right = array_struct[HEIGHT - 2][1].active;

    array_struct[HEIGHT - 1][0].left = array_struct[HEIGHT - 1][WIDTH - 1].active;
    array_struct[HEIGHT - 1][0].right = array_struct[HEIGHT - 1][1].active;

    array_struct[HEIGHT - 1][0].lower_left = array_struct[0][WIDTH - 1].active;
    array_struct[HEIGHT - 1][0].lower = array_struct[0][0].active;
    array_struct[HEIGHT - 1][0].lower_right = array_struct[0][1].active;

    array_struct[HEIGHT - 1][WIDTH - 1].upper_left = array_struct[HEIGHT - 2][WIDTH - 2].active;
    array_struct[HEIGHT - 1][WIDTH - 1].upper = array_struct[HEIGHT - 2][WIDTH - 1].active;
    array_struct[HEIGHT - 1][WIDTH - 1].upper_right = array_struct[HEIGHT - 2][0].active;

    array_struct[HEIGHT - 1][WIDTH - 1].left = array_struct[HEIGHT - 1][WIDTH - 2].active;
    array_struct[HEIGHT - 1][WIDTH - 1].right = array_struct[HEIGHT - 1][0].active;

    array_struct[HEIGHT - 1][WIDTH - 1].lower_left = array_struct[0][WIDTH - 2].active;
    array_struct[HEIGHT - 1][WIDTH - 1].lower = array_struct[0][WIDTH - 1].active;
    array_struct[HEIGHT - 1][WIDTH - 1].lower_right = array_struct[0][0].active;

     for i in 0..HEIGHT {
        for k in 0..WIDTH {
            actives[i][k]=array_struct[i][k].active;
           
        }
     }
      
}

fn next_turn(array_struct: &mut Vec<Vec<Field>>) {
    for i in 0..HEIGHT {
        for k in 0..WIDTH {
            let mut counter = 0;

            if array_struct[i][k].upper_left == true {
                counter += 1;
            }
            if array_struct[i][k].upper == true {
                counter += 1;
            }
            if array_struct[i][k].upper_right == true {
                counter += 1;
            }

            if array_struct[i][k].left == true {
                counter += 1;
            }
            if array_struct[i][k].right == true {
                counter += 1;
            }

            if array_struct[i][k].lower_left == true {
                counter += 1;
            }
            if array_struct[i][k].lower == true {
                counter += 1;
            }
            if array_struct[i][k].lower_right == true {
                counter += 1;
            }

            if array_struct[i][k].active == false {
                if counter == 3 {
                    array_struct[i][k].active = true;
                }
            }

            if array_struct[i][k].active == true {
                if counter <= 1 {
                    array_struct[i][k].active = false;
                } else if counter >= 4 {
                    array_struct[i][k].active = false;
                }
            }
        }
    }
}
