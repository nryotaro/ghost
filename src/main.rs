extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{ApplicationWindow, Builder, Box, Label};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("ghost.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get widnow");
    window.set_application(Some(application));

    // let label: Label = builder.get_object("label1").expect("Couldn't get label");
    let label: Label = Label::new(Some("doge"));
    let menu: Box = builder.get_object("list").expect("Couldn't get box");
    menu.add(&label);
    // list
    // https://palepoli.skr.jp/tips/pygobject/container2.php
    //menu.remove(&label);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
	Some("dev.nryotaro.ghost"),
	Default::default()
    ).expect("Initialization failed...");
    
    application.connect_activate(|app| {
	build_ui(app)
    });

    application.run(&args().collect::<Vec<_>>());
}
