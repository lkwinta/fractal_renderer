use std::cell::RefCell;
use std::rc::Rc;
use imgui::{Drag, Ui};
use crate::ui::event_observer::FractalType::{Julia, Mandelbrot};
use crate::ui::event_observer::{Observable, Observer, ObserverEvent::{FractalIterations, FractalChoice, FractalAxisRange}, ObserverEvent};
use crate::ui::event_observer::ObserverEvent::FractalHSVScaleChange;

pub struct PropertiesWindow {
    items: Vec<String>,
    selected_index: usize,

    julia_constant: [f32; 2],
    hsv_scale: [f32; 3],
    zoom: f32,
    focus: [f32; 2],
    camera_width: f32,
    camera_height: f32,

    real_x_axis_range: [f32; 2],
    real_y_axis_range: [f32; 2],
    max_iterations: i32,

    current_width: i32,
    current_height: i32,
    lock_aspect_ratio: bool,

    observers: Vec<Rc<RefCell<dyn Observer>>>
}

impl Default for PropertiesWindow {
    fn default() -> Self {
        let items = vec!["Mandelbrot".into(), "Julia".into()];
        Self {
            selected_index: 0,
            items,

            julia_constant: [-0.8, 0.156],
            hsv_scale: [1.0, 1.0, 1.0],
            zoom: 1.0,
            focus: [0.0, 0.0],
            camera_width: 2.0,
            camera_height: 2.0,

            real_x_axis_range: [-1.0, 1.0],
            real_y_axis_range: [-1.0, 1.0],
            max_iterations: 500,

            current_width: 800,
            current_height: 600,
            lock_aspect_ratio: true,

            observers: Vec::new()
        }
    }
}

impl PropertiesWindow {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.window("Properties")
            .size([300.0, 300.0], imgui::Condition::FirstUseEver)
            .movable(false)
            .position([self.current_width as f32, 0.0], imgui::Condition::Always)
            .position_pivot([1.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .build(|| {
                if Drag::new("##max_iterations").display_format("Max iterations: %d").speed(1.0).build(ui, &mut self.max_iterations) {
                    self.notify_observers(FractalIterations(self.max_iterations))
                }

                self.draw_fractal_combo(ui);
                self.handle_fractal_constant(ui);
                self.draw_hsv_scale(ui);

                ui.text("Focus point");
                {
                    ui.set_next_item_width(-1.0);
                    let _item_width_stack_token = ui.push_item_width(ui.calc_item_width()/2.0);
                    Drag::new("##focus.x").display_format("X: %f").speed(0.001).build(ui, &mut self.focus[0]);
                    ui.same_line();
                    Drag::new("##focus.y").display_format("Y: %f").speed(0.001).build(ui, &mut self.focus[1]);
                }

                ui.text("Zoom level");
                ui.same_line();
                ui.set_next_item_width(-1.0);
                Drag::new("##zoom").display_format("%f").speed(0.1).build(ui, &mut self.zoom);

                self.draw_camera_size(ui);
            });



        self.real_x_axis_range[0] = self.focus[0] - self.camera_width / 2.0 / self.zoom;
        self.real_x_axis_range[1] = self.focus[0] + self.camera_width / 2.0 / self.zoom;
        self.real_y_axis_range[0] = self.focus[1] - self.camera_height / 2.0 / self.zoom;
        self.real_y_axis_range[1] = self.focus[1] + self.camera_height / 2.0 / self.zoom;

        if !ui.io().want_capture_mouse {
            self.notify_observers(FractalAxisRange{x: self.real_x_axis_range, y: self.real_y_axis_range});
        }
    }

    fn draw_fractal_combo(&mut self, ui: &Ui) {
        ui.text("Fractal");
        ui.same_line();
        ui.set_next_item_width(-1.0);
        if let Some(_cb) = ui.begin_combo("##fractal_combo", &self.items[self.selected_index]) {
            for cur in &self.items {
                if &self.items[self.selected_index] == cur {
                    // Auto-scroll to selected item
                    ui.set_item_default_focus();
                }
                // Create a "selectable"
                let clicked = ui.selectable_config(cur)
                    .selected(&self.items[self.selected_index] == cur)
                    .build();
                // When item is clicked, store it
                if clicked {
                    self.selected_index = self.items.iter().position(|item| item == cur).unwrap();
                }
            }
        }
    }

    fn handle_fractal_constant(&mut self, ui: &Ui) {
        if self.items[self.selected_index] == "Julia" {
            ui.text("Julia constant");

            ui.set_next_item_width(-1.0);
            let _item_width_stack_token = ui.push_item_width(ui.calc_item_width() / 2.0);
            Drag::new("##c.x").display_format("X: %f").speed(0.001).build(ui, &mut self.julia_constant[0]);
            ui.same_line();
            Drag::new("##c.y").display_format("Y: %f").speed(0.001).build(ui, &mut self.julia_constant[1]);

            self.notify_observers(FractalChoice(Julia(self.julia_constant)));
        } else {
            self.notify_observers(FractalChoice(Mandelbrot));
        }
    }

    fn draw_camera_size(&mut self, ui: &Ui) {
        ui.text("Camera size");
        if ui.checkbox("Lock aspect ratio", &mut self.lock_aspect_ratio) {
            if self.lock_aspect_ratio {
                self.camera_height = self.current_height as f32/self.current_width as f32 * self.camera_width
            }
        }

        ui.set_next_item_width(-1.0);
        let _item_width_stack_token = ui.push_item_width(ui.calc_item_width()/2.0);
        if Drag::new("##camera.width").display_format("Width: %f").speed(0.1).build(ui, &mut self.camera_width) && self.lock_aspect_ratio {
            self.camera_height = self.current_height as f32/self.current_width as f32 * self.camera_width
        }
        ui.same_line();
        if Drag::new("##camera.height").display_format("Height: %f").speed(0.1).build(ui, &mut self.camera_height) && self.lock_aspect_ratio {
            self.camera_width = self.current_width as f32/self.current_height as f32 * self.camera_height
        }
    }

    fn draw_hsv_scale(&mut self, ui: &Ui) {
        ui.text("HSV scale");
        ui.set_next_item_width(-1.0);
        let _item_width_stack_token = ui.push_item_width(ui.calc_item_width() / 3.0);
        let h = Drag::new("##h_scale").display_format("H: %f").speed(0.001).build(ui, &mut self.hsv_scale[0]);
        ui.same_line();
        let s = Drag::new("##s_scale").display_format("S: %f").speed(0.001).build(ui, &mut self.hsv_scale[1]);
        ui.same_line();
        let v = Drag::new("##v_scale").display_format("V: %f").speed(0.001).build(ui, &mut self.hsv_scale[2]);

        if h || s || v {
            self.notify_observers(FractalHSVScaleChange{h: self.hsv_scale[0], s: self.hsv_scale[1], v: self.hsv_scale[2]});
        }
    }
}

impl Observer for PropertiesWindow {
    fn notify(&mut self, event: &ObserverEvent) {
        match event {
            ObserverEvent::Zoom(zoom) => self.zoom *= zoom,
            ObserverEvent::UnZoom(zoom) => self.zoom /= zoom,
            ObserverEvent::Translate{xrel, yrel} => {
                self.focus[0] += *xrel as f32 / self.current_width as f32 / self.zoom;
                self.focus[1] += *yrel as f32 / self.current_height as f32 / self.zoom;
            },
            ObserverEvent::WindowSizeChanged {width, height} => {
                self.current_width = *width;
                self.current_height = *height;

                if self.lock_aspect_ratio {
                    self.camera_width = self.current_width as f32/self.current_height as f32 * self.camera_height
                }
            }
            _ => { eprint!("Received unknown event in properties_window!") }
        }
    }
}

impl Observable<'_> for PropertiesWindow {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer)
    }

    fn notify_observers(&mut self, event: ObserverEvent) {
        for observer in self.observers.iter() {
            observer.borrow_mut().notify(&event)
        }
    }
}