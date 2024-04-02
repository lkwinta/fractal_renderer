mod resources;
mod renderer;

extern crate sdl2;

use sdl2::keyboard::Keycode;

use imgui::{Context, Drag, TableFlags};
use imgui_sdl2_support::SdlPlatform;
use imgui_glow_renderer::glow;
use imgui_glow_renderer::glow::HasContext;
use imgui_glow_renderer::AutoRenderer;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};
use crate::renderer::MandelbrotRenderer;
use gl;
use sdl2::sys::Cursor;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Debug)]
enum Fractal {
    Mandelbrot,
    Julia(f32, f32),
}


fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("Mandelbrot set visualization", WIDTH, HEIGHT)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    window.gl_make_current(&gl_context).map_err(|e| e.to_string())?;
    window.subsystem().gl_set_swap_interval(sdl2::video::SwapInterval::VSync).map_err(|e| e.to_string())?;

    let gl= glow_context(&window);
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui.style_mut().window_rounding = 10.0;

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mandelbrot_renderer = MandelbrotRenderer::new();

    let mut event_pump = sdl_context.event_pump()?;

    let items = vec!["Mandelbrot", "Julia"];
    let mut selected = &items[0];

    let mut julia_constant = [-0.8, 0.156];
    let mut zoom = 1.0;
    let mut focus = [0.0, 0.0];
    let mut camera_width = 2.0;
    let mut camera_height = 2.0;

    let mut real_x_axis_range = [-1.0, 1.0];
    let mut real_y_axis_range = [-1.0, 1.0];
    let mut max_iterations = 500;

    let mut left_btn_down = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseWheel {y, ..} =>
                {
                    if y > 0 {
                        zoom *= 1.1;
                    } else {
                        zoom /= 1.1;
                    }
                },
                Event::MouseButtonDown {mouse_btn, ..} => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        left_btn_down = true;
                    }
                },
                Event::MouseButtonUp {mouse_btn, ..} => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        left_btn_down = false;
                    }
                },
                Event::MouseMotion {xrel, yrel, ..} => {
                    // if event_pump.mouse_state().left() // nie działa bo event pump porzyczone jako mutable w poll event kurwa
                    if left_btn_down {
                        focus[0] += -xrel as f32 / window.size().0 as f32 / zoom;
                        focus[1] += yrel as f32  / window.size().1 as f32 / zoom;
                    }
                }
                _ => {}
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        /* create imgui UI here */

        ui.window("Properties")
            .size([300.0, 300.0], imgui::Condition::FirstUseEver)
            .movable(false)
            .position([window.size().0 as f32, 0.0], imgui::Condition::Always)
            .position_pivot([1.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .build(|| {
                ui.text("Fractal");
                ui.same_line();
                ui.set_next_item_width(-1.0);
                if let Some(_cb) = ui.begin_combo("Fractal", selected) {
                    for cur in &items {
                        if selected == cur {
                            // Auto-scroll to selected item
                            ui.set_item_default_focus();
                        }
                        // Create a "selectable"
                        let clicked = ui.selectable_config(cur)
                            .selected(selected == cur)
                            .build();
                        // When item is clicked, store it
                        if clicked {
                            selected = cur;
                        }
                    }
                }
                if *selected == "Julia" {
                    ui.text("Julia constant");

                    {
                        ui.set_next_item_width(-1.0);
                        let _item_width_stack_token = ui.push_item_width(ui.calc_item_width()/2.0);
                        Drag::new("##c.x").display_format("X: %f").speed(0.001).build(ui, &mut julia_constant[0]);
                        ui.same_line();
                        Drag::new("##c.y").display_format("Y: %f").speed(0.001).build(ui, &mut julia_constant[1]);
                    }


                    mandelbrot_renderer.set_julia(true);
                    mandelbrot_renderer.set_julia_constant(julia_constant[0], julia_constant[1]);
                } else {
                    mandelbrot_renderer.set_julia(false);
                }

                ui.text("Focus point");
                {
                    ui.set_next_item_width(-1.0);
                    let _item_width_stack_token = ui.push_item_width(ui.calc_item_width()/2.0);
                    Drag::new("##focus.x").display_format("X: %f").speed(0.001).build(ui, &mut focus[0]);
                    ui.same_line();
                    Drag::new("##focus.y").display_format("Y: %f").speed(0.001).build(ui, &mut focus[1]);
                }

                ui.text("Zoom level");
                ui.same_line();
                ui.set_next_item_width(-1.0);
                Drag::new("##zoom").display_format("%f").speed(0.1).build(ui, &mut zoom);

                ui.text("Camera size");
                {
                    ui.set_next_item_width(-1.0);
                    let _item_width_stack_token = ui.push_item_width(ui.calc_item_width()/2.0);
                    Drag::new("##camera.width").display_format("Width: %f").speed(0.1).build(ui, &mut camera_width);
                    ui.same_line();
                    Drag::new("##camera.height").display_format("Height: %f").speed(0.1).build(ui, &mut camera_height);
                }

                real_x_axis_range[0] = focus[0] - camera_width/2.0/zoom;
                real_x_axis_range[1] = focus[0] + camera_width/2.0/zoom;
                real_y_axis_range[0] = focus[1] - camera_height/2.0/zoom;
                real_y_axis_range[1] = focus[1] + camera_height/2.0/zoom;

                mandelbrot_renderer.set_x_axis_range(real_x_axis_range[0], real_x_axis_range[1]);
                mandelbrot_renderer.set_y_axis_range(real_y_axis_range[0], real_y_axis_range[1]);

                Drag::new("##max_iterations").display_format("Max iterations: %d").speed(1.0).build(ui, &mut max_iterations);
                mandelbrot_renderer.set_max_iterations(max_iterations);
            });

         ui.show_metrics_window(&mut true);
        // ui.show_default_style_editor();
        //ui.show_demo_window(&mut true);

        /* render */
        let draw_data = imgui.render();

        unsafe {
            gl::Viewport(0, 0, window.size().0 as i32, window.size().1 as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        mandelbrot_renderer.render(window.size().0 as f32, window.size().1 as f32);
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }

    Ok(())
}

