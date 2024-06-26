use std::cell::RefCell;
use std::rc::Rc;
use crate::renderer::FractalRenderer;
use crate::ui::input_handler::InputHandler;
use crate::ui::properties_window::PropertiesWindow;
use crate::ui::event_observer::Observable;
use crate::ui::window::Window;

/// Main application struct
/// holds the window, fractal renderer, properties window and input handler
/// and handles events and rendering
/// sets up the window, fractal renderer, properties window and input handler
/// and registers the properties window and fractal renderer as observers
pub struct Application {
    window: Window,
    fractal_renderer: Rc<RefCell<FractalRenderer>>,
    properties_window: Rc<RefCell<PropertiesWindow>>,
    input_handler: InputHandler,
}

impl Application {
    /// Creates a new application
    pub fn new() -> Application {
        let window = Window::new();

        // Rc is a reference-counted box for sharing the properties window and fractal renderer
        // between the input handler and the window
        let fractal_renderer = Rc::new(RefCell::new(FractalRenderer::new()));
        let properties_window = Rc::new(RefCell::new(PropertiesWindow::default()));
        let mut input_handler = InputHandler::default();

        // register the fractal renderer and properties window as observers
        properties_window.borrow_mut().register_observer(fractal_renderer.clone());
        input_handler.register_observer(properties_window.clone());

        Self {
            window,
            fractal_renderer: fractal_renderer.clone(),
            properties_window: properties_window.clone(),
            input_handler,
        }
    }

    // Process handling events
    pub fn handle_events(&mut self) -> bool {
        let events = self.window.handle_events();
        for event in events {
            if !self.input_handler.handle_input(&event) { return false }
        }

        true
    }

    // Render the window
    pub fn render(&mut self) {
        // Clear the viewport
        unsafe {
            gl::Viewport(0, 0, self.window.window.size().0 as i32, self.window.window.size().1 as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        // Render imgui gui and fractal
        self.window.platform.prepare_frame(&mut self.window.imgui, &mut self.window.window, &mut self.window.event_pump);
        let mut ui = self.window.imgui.new_frame();
        self.properties_window.borrow_mut().draw(&mut ui);
        let draw_data = self.window.imgui.render();

        self.fractal_renderer.borrow_mut().render(self.window.window.size().0 as f32, self.window.window.size().1 as f32);

        self.window.renderer.render(draw_data).unwrap();
        self.window.window.gl_swap_window();
    }
}

