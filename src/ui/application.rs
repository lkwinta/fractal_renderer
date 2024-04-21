use imgui::Context;
use imgui_glow_renderer::{AutoRenderer, glow};
use imgui_sdl2_support::SdlPlatform;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::video::{GLContext, GLProfile, Window};
use crate::renderer::FractalRenderer;
use crate::ui::input_handler::InputHandler;
use crate::ui::properties_window::PropertiesWindow;
use crate::ui::state_instructions::StateInstruction::Close;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub struct Application<'a> {
    pub window: Window,
    #[allow(dead_code)] sdl_context: Sdl,
    #[allow(dead_code)] video_subsystem: VideoSubsystem,
    #[allow(dead_code)] gl_context: GLContext,
    pub imgui: Context,
    pub platform: SdlPlatform,
    pub renderer: AutoRenderer,
    pub event_pump: EventPump,
    fractal_renderer: FractalRenderer,
    properties_window: PropertiesWindow<'a>,
    input_handler: InputHandler
}

impl Application<'_> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init()
            .expect("Failed to create SDL context");
        let video_subsystem = sdl_context.video()
            .expect("Failed to create SDL VideoSubsystem");

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(4, 4);
        gl_attr.set_context_profile(GLProfile::Core);

        let window = video_subsystem
            .window("Mandelbrot set visualization", WIDTH, HEIGHT)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create SDL Window");

        let gl_context = window.gl_create_context().map_err(|e| e.to_string())
            .expect("Failed to create opengl context");
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

        window.gl_make_current(&gl_context).map_err(|e| e.to_string())
            .expect("Failed to switch current opengl context");
        window.subsystem().gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
            .map_err(|e| e.to_string()).expect("Failed to setup VSync");

        let mut imgui = setup_imgui();

        Self {
            video_subsystem,
            gl_context,
            platform: SdlPlatform::init(&mut imgui),
            renderer: AutoRenderer::initialize(glow_context(&window), &mut imgui)
                .expect("Failed to create imgui AutoRenderer"),
            event_pump: sdl_context.event_pump()
                .expect("Failed to create SDL event pump"),
            imgui,
            window,
            sdl_context,
            fractal_renderer: FractalRenderer::new(),
            properties_window: PropertiesWindow::default(),
            input_handler: InputHandler::default()
        }
    }

    pub fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            self.platform.handle_event(&mut self.imgui, &event);

            match self.input_handler.handle_input(&event) {
                Some(Close) => { return false; },
                Some(instruction) => self.properties_window.handle_instruction(&instruction),
                _ => {}
            }
        }

        true
    }

    pub fn render(&mut self) {
        self.platform.prepare_frame(&mut self.imgui, &mut self.window, &mut self.event_pump);

        let mut ui = self.imgui.new_frame();

        self.properties_window.draw(&mut ui).iter().for_each(|instruction| self.fractal_renderer.handle_instruction(instruction));
        ui.show_metrics_window(&mut true);

        let draw_data = self.imgui.render();

        unsafe {
            gl::Viewport(0, 0, self.window.size().0 as i32, self.window.size().1 as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        self.fractal_renderer.render(self.window.size().0 as f32, self.window.size().1 as f32);

        self.renderer.render(draw_data).unwrap();

        self.window.gl_swap_window();
    }
}

fn setup_imgui() -> Context {
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui.style_mut().window_rounding = 10.0;
    imgui
}

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}