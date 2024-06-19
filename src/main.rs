mod resources;
mod renderer;
mod ui;
mod application;

pub fn main() {
    let mut application = application::Application::new();

    loop {
        if !application.handle_events() {
            break;
        }
        application.render();
    }
}

