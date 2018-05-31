extern crate gio;
extern crate gtk;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Grid, Label, PositionType};
use std::env::args;
use std::path::Path;

mod cmd;
mod docker_json;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let glade_src = Path::new("./grid.glade");
    let builder = Builder::new_from_file(glade_src);
    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);

    window.set_title("Dokka - Your friendly Docker");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1000, 400);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let json_output = cmd::docker_ps().expect("could not exec command");
    let dockers = docker_json::parse_cmd(json_output).expect("could not translate dockers");
    let grid: Grid = builder.get_object("grid").expect("Couldn't get grid");
    let mut i = 0;
    let mut acc: Box<Option<Box<Label>>> = Box::new(None);
    let width = 50;
    let height = 30;

    for docker in dockers {
        let name: &str = &docker.id.unwrap_or(String::from("")).to_owned();
        let image: &str = &docker.image.unwrap_or(String::from("")).to_owned();
        let created_at: &str = &docker.created_at.unwrap_or(String::from("")).to_owned();
        let ports: &str = &docker.ports.unwrap_or(String::from("")).to_owned();
        let status: &str = &docker.status.unwrap_or(String::from("")).to_owned();
        let label1 = Label::new(Some(name));
        let label2 = Label::new(Some(image));
        let label3 = Label::new(Some(created_at));
        let label4 = Label::new(Some(ports));
        let label5 = Label::new(Some(status));
        label1.set_selectable(true);
        label2.set_selectable(true);

        if i == 0 {
            grid.attach(&label1, 0, 0, width, height);
        } else {
            match acc.as_ref() {
                Some(acc_label_box) => {
                    let acc_label = acc_label_box.as_ref();
                    grid.attach_next_to(&label1, acc_label, PositionType::Bottom, width, height);
                }
                _ => {}
            }
        }

        grid.attach_next_to(&label2, &label1, PositionType::Right, width, height);
        grid.attach_next_to(&label3, &label2, PositionType::Right, width, height);
        grid.attach_next_to(&label4, &label3, PositionType::Right, width, height);
        grid.attach_next_to(&label5, &label4, PositionType::Right, width, height);

        i += 1;
        let cl = label1.clone();
        let boxed = Box::new(Some(Box::new(cl)));
        acc = boxed;
    }

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
