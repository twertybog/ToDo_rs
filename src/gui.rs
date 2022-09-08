use fltk::enums::{Color, Align};
use fltk::prelude::*;
use fltk::*;
use crate::ToDo;

pub fn gui(todo: &mut ToDo){
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut my_window = window::Window::new(100, 100, 800, 600, "ToDo");
        let mut scroll = group::Scroll::default().with_size(800, 600);
        
        scroll.set_color(Color::DarkCyan);
        scroll.set_type(group::ScrollType::Vertical);
        scroll.set_scrollbar_size(5);

        let mut add = button::Button::default()
            .with_size(40, 40)
            .with_label("+");

        let mut remove = button::Button::default()
            .with_size(40, 40)
            .right_of(&add, 40).with_label("-");

        add.set_color(Color::DarkRed);
        remove.set_color(Color::DarkGreen);
        let mut pack = group::Pack::default_fill().center_of(&scroll);
        pack.set_spacing(10);
        frame::Frame::default().with_size(0, 40);

        for i in &mut todo.tasks{
            let check = button::CheckButton::default();
            check.set_checked(*i.1);
            let mut input = input::Input::default()
                .with_size(300, 40)
                .with_align(Align::Left);
            input.append(i.0); 
        }

        pack.end();

        my_window.end();
        my_window.show();
        app.run().unwrap();
}