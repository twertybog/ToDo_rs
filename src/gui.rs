use crate::ToDo;
use crate::act::actions;
use fltk::enums::Color;
use fltk::prelude::*;
use fltk::*;

#[derive(Clone, Copy)]
enum Message {
    Add,
    Save,
    Remove,
}

pub fn gui() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut todo = Box::new(ToDo::new().expect("Initialisation of db failed"));
    let mut window = window::Window::new(100, 100, 600, 500, "ToDo");
    window.set_color(Color::DarkCyan);

    let mut scroll = group::Scroll::default_fill();
    scroll.set_color(Color::DarkCyan);
    scroll.set_scrollbar_size(5);
    let mut save = button::Button::new(10, 10, 75, 40, "Save");
    save.set_color(Color::from_rgb(70, 195, 242));
    let mut remove = button::Button::new(95, 10, 75, 40, "Remove\ndone");
    remove.set_color(Color::from_rgb(201, 0, 55));
    let mut input = input::Input::new(10, 60, 500, 40, "");
    input.set_text_size(16);
    let mut add = button::Button::new(515, 60, 65, 40, "Add");
    add.set_color(Color::DarkYellow);

    let mut spacer: i32 = 120;

    let mut plans = Box::new(Vec::new());
    drawer(&mut todo, &mut plans, &mut spacer, &mut scroll);
    window.end();
    window.show();

    let (s, r) = app::channel::<Message>();

    add.emit(s, Message::Add);
    save.emit(s, Message::Save);
    remove.emit(s, Message::Remove);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Add => {
                    if input.value().is_empty() != true{
                        let checkbutt = button::CheckButton::new(10, spacer + 10,
                            20, 20, "");
                        let mut text = input::Input::new(40, spacer, 540, 40, "");
                        text.set_value(&input.value());
                        input.set_value("");
                        scroll.add(&checkbutt);
                        scroll.add(&text);
                        plans.as_mut().push((checkbutt, text));
                        scroll.redraw();
                        window.redraw();
                        spacer += 50;
                    }
                }
                Message::Save => {
                    todo.tasks = ToDo::remove_all();

                    for (button, text) in plans.as_ref(){
                        todo.tasks.insert(text.value(), button.value());
                    }

                    actions("save", todo.tasks.clone()).unwrap();
                },
                Message::Remove => {
                    todo.tasks = ToDo::remove_all();
                    for i in (0..plans.len()).rev(){
                        if plans[i].0.value() == false{
                            todo.tasks.insert(plans[i].1.value(), plans[i].0.value());
                        }
                        scroll.remove(&plans[i].0);
                        scroll.remove(&plans[i].1);
                    }
                    spacer = 120;
                    plans.clear();
                    drawer(&mut todo, &mut plans, &mut spacer, &mut scroll);
                    scroll.redraw();
                    window.redraw();                  
                },
            }
        }
    }

    app.run().unwrap();
}

fn drawer(todo: &mut ToDo,
    plans: &mut Box<Vec<(button::CheckButton, input::Input)>>,
    spacer: &mut i32,
    scroll: &mut group::Scroll
){
    for i in &todo.tasks {
        let checkbutt = button::CheckButton::new(10, *spacer + 10, 20, 20, "");
        let mut text = input::Input::new(40, *spacer, 540, 40, "");
        text.set_value(i.0);
        checkbutt.set_checked(*i.1);
        plans.as_mut().push((checkbutt.clone(), text.clone()));
        scroll.add(&checkbutt);
        scroll.add(&text);
        *spacer += 50;
    }
}
