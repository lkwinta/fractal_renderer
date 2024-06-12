use std::cell::RefCell;
use std::rc::Rc;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use crate::ui::event_observer::{Observable, Observer, ObserverEvent};


pub struct InputHandler {
    left_btn_down: bool,
    observers: Vec<Rc<RefCell<dyn Observer>>>
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            left_btn_down: false,
            observers: Vec::new()
        }
    }
}

impl InputHandler {
    pub fn handle_input(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            Event::MouseWheel {y, ..} =>
                {
                    if y > &0 {
                        self.notify_observers(ObserverEvent::Zoom(1.1))
                    } else {
                        self.notify_observers(ObserverEvent::UnZoom(1.1))
                    }
                },
            Event::MouseButtonDown {mouse_btn, ..} => {
                if *mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_btn_down = true;
                }
            },
            Event::MouseButtonUp {mouse_btn, ..} => {
                if *mouse_btn == sdl2::mouse::MouseButton::Left {
                    self.left_btn_down = false;
                }
            },
            Event::MouseMotion {xrel, yrel, ..} => {
                if self.left_btn_down {
                    self.notify_observers(ObserverEvent::Translate{ xrel: -*xrel, yrel: *yrel});
                }
            },
            Event::Window {
                win_event, ..
            } => {
                match win_event {
                    WindowEvent::Resized(width, height) => {
                        self.notify_observers(ObserverEvent::WindowSizeChanged{width: *width, height: *height});
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        true
    }
}

impl Observable<'_> for InputHandler {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer)
    }

    fn notify_observers(&mut self, event: ObserverEvent) {
        for observer in self.observers.iter() {
            observer.borrow_mut().notify(&event)
        }
    }
}