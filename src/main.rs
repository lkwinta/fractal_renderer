use crate::ui::Application;

mod resources;
mod renderer;
mod ui;

pub fn main() {
    let mut application = Application::new();

    loop {
        if !application.handle_events() {
            break;
        }
        application.render();
    }
}

