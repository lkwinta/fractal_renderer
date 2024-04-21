use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::ui::state_instructions::StateInstruction;


pub struct InputHandler {
    left_btn_down: bool
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            left_btn_down: false
        }
    }
}

impl InputHandler {
    pub fn handle_input(&mut self, event: &Event) -> Option<StateInstruction> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Some(StateInstruction::Close),
            Event::MouseWheel {y, ..} =>
                {
                    return if y > &0 {
                        Some(StateInstruction::Zoom(1.1))
                    } else {
                        Some(StateInstruction::UnZoom(1.1))
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
                    return Some(StateInstruction::Translate{ xrel: -*xrel, yrel: *yrel});

                }
            }
            _ => {}
        }

        None
    }
}