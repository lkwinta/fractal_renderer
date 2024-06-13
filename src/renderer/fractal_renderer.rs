use std::path::Path;

use gl;
use gl::types::{GLuint, GLvoid};

use crate::ui::event_observer::{FractalType, Observer, ObserverEvent};
use crate::renderer::{Program, Shader};
use crate::resources::Resources;

/// OpenGL wrapper around the fractal rendering shader program
/// hides the details of the shader program and provides a simple interface to set the uniforms
pub struct FractalRenderer {
    program: Program,

    vertex_array: GLuint,
    vertex_buffer: GLuint,
    element_buffer: GLuint,
}

impl FractalRenderer {
    const VERTICES: [f32; 12] = [
        -1.0, -1.0, 0.0,
        1.0, 1.0, 0.0,
        -1.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
    ];

    //  2---,1
    //  | .' |
    //  0'---3
    const INDICES: [i32; 6] = [
        0, 1, 2,
        0, 3, 1
    ];

    pub fn new() -> Self {
        let shaders_resources = Resources::from_relative_exe_path(Path::new("assets\\shaders")).unwrap();

        let vertex_shader = Shader::from_resources(
            &shaders_resources,
            "mandelbrot.vert",
            gl::VERTEX_SHADER).unwrap();

        let fragment_shader = Shader::from_resources(
            &shaders_resources,
            "mandelbrot.frag",
            gl::FRAGMENT_SHADER).unwrap();

        let program = Program::from_shaders(
            &[&vertex_shader, &fragment_shader]).unwrap();

        let mut vertex_array = 0;
        let mut vertex_buffer= 0;
        let mut element_buffer = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::GenBuffers(1, &mut element_buffer);

            gl::BindVertexArray(vertex_array);

            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (Self::VERTICES.len() * std::mem::size_of::<f32>()) as isize,
                Self::VERTICES.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (Self::INDICES.len() * std::mem::size_of::<i32>()) as isize,
                Self::INDICES.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    3 * std::mem::size_of::<f32>() as i32,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(0);


            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self {
            program,
            vertex_array,
            vertex_buffer,
            element_buffer,
        }
    }

    pub fn set_x_axis_range(&self, x: f32, y: f32) {
        self.program.use_program();
        self.program.set_f32_2(c"x_axis_range", x, y).unwrap();
    }
    pub fn set_y_axis_range(&self, x: f32, y: f32) {
        self.program.use_program();
        self.program.set_f32_2(c"y_axis_range", x, y).unwrap();
    }

    pub fn set_julia(&self, enabled: bool) {
        self.program.use_program();
        self.program.set_bool(c"julia", enabled).unwrap();
    }

    pub fn set_julia_constant(&self, x: f32, y: f32) {
        self.program.use_program();
        self.program.set_f32_2(c"julia_const", x, y).unwrap();
    }

    pub fn set_max_iterations(&self, iterations: i32) {
        self.program.use_program();
        self.program.set_i32(c"max_iterations", iterations).unwrap();
    }

    pub fn set_hsv_scale(&self, h: f32, s: f32, v: f32) {
        self.program.use_program();
        self.program.set_f32_3(c"hsv_scale", h, s, v).unwrap();
    }

    pub fn set_terminal_color(&self, h: f32, s: f32, v: f32) {
        self.program.use_program();
        self.program.set_f32_3(c"terminal_color", h, s, v).unwrap();
    }

    pub fn render(&self, x: f32, y: f32) {
        self.program.use_program();
        self.program.set_f32_2(c"screen_size", x, y).unwrap();

        unsafe {
            gl::BindVertexArray(self.vertex_array);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl Observer for FractalRenderer {
    fn notify(&mut self, event: &ObserverEvent) {
        match event {
            ObserverEvent::FractalIterations(iterations) => self.set_max_iterations(*iterations),
            ObserverEvent::FractalChoice(fractal) => {
                match fractal {
                    FractalType::Julia(constant) => {
                        self.set_julia(true);
                        self.set_julia_constant(constant[0], constant[1]);
                    }
                    FractalType::Mandelbrot => self.set_julia(false)
                }
            }
            ObserverEvent::FractalAxisRange { x, y } => {
                self.set_x_axis_range(x[0], x[1]);
                self.set_y_axis_range(y[0], y[1]);
            },
            ObserverEvent::FractalHSVScaleChange { h, s, v } => self.set_hsv_scale(*h, *s, *v),
            ObserverEvent::FractalTerminalColorChange { r,g, b } => self.set_terminal_color(*r, *g, *b),
            _ => { eprint!("Received unknown event in fractal_renderer!") }
        }
    }
}

impl Drop for FractalRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array);
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.element_buffer);
        }
    }
}