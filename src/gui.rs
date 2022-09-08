use fltk::enums::{Color};
use fltk::prelude::*;
use fltk::*;
use crate::ToDo;

pub fn gui(todo: &mut ToDo){
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut my_window = window::Window::new(100, 100, 400, 300, "ToDo");
        my_window.set_color(Color::DarkCyan);
        let mut add = button::Button::default().with_size(40, 40).with_label("+");
        add.set_color(Color::DarkRed);
        let mut pack = group::Pack::default_fill();
        pack.set_spacing(10);
        frame::Frame::default().with_size(0, 40);

        for i in &mut todo.tasks{
            button::CheckButton::default().set_checked(*i.1);
            let _input = input::Input::default().with_size(40, 40).append(&i.0);
        }

        pack.end();

        my_window.end();
        my_window.show();
        app.run().unwrap();
}