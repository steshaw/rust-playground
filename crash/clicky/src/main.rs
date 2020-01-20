use gio::prelude::*;
use gtk::prelude::*;

use gtk::Button;
use gtk::{Application, ApplicationWindow};
use gtk::{Window, WindowType};

use std::fs::File;
use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::io::Write;

fn write_file() {
    let file_name = "clicky.log";
    let mut file = File::create(file_name).expect(&format!("cannot open {} :-(", file_name));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)
        .expect(&format!("Cannot open {} for append", file_name));
    file.write_all(b"Hello, world!\n")
        .expect("cannot write to file :-(");
    file.flush().expect("Cannot flush file");
}

fn old_way() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
        write_file();
    });

    gtk::main();
}

fn new_way() {
    let app = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("Failed to initialise GTK application");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Application");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);
        window.show_all();
    });

    app.run(&[]);
}

fn main() {
    let enable_new_way = false;
    if enable_new_way {
        new_way();
    } else {
        old_way();
    }
}
