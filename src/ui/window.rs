use imgui::Context;
use imgui_glow_renderer::{AutoRenderer, glow};
use imgui_sdl2_support::SdlPlatform;
use sdl2::video::{GLContext, GLProfile, WindowBuildError};
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, EventPollIterator};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub struct Window {
    pub window: sdl2::video::Window,
    pub imgui: Context,
    pub platform: SdlPlatform,
    pub renderer: AutoRenderer,
    pub event_pump: EventPump,

    /**
        References that must exist but are not used
     */
    #[allow(dead_code)] sdl_context: Sdl,
    #[allow(dead_code)] video_subsystem: VideoSubsystem,
    #[allow(dead_code)] gl_context: GLContext,
}

impl Window {
    pub fn new() -> Self {
        let sdl_context = sdl2::init()
            .expect("Failed to create SDL context");
        let video_subsystem = sdl_context.video()
            .expect("Failed to create SDL VideoSubsystem");

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(4, 4);
        gl_attr.set_context_profile(GLProfile::Core);

        let window = create_window(&video_subsystem)
            .map_err(|e| e.to_string())
            .expect("Failed to create SDL Window");
        let gl_context = setup_gl_context(&window, &video_subsystem);

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
        }
    }

    pub fn handle_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        for event in self.event_pump.poll_iter() {
            self.platform.handle_event(&mut self.imgui, &event);
            events.push(event);
        }

        events
    }
}

fn create_window(video_subsystem: &VideoSubsystem) -> Result<sdl2::video::Window, WindowBuildError> {
    video_subsystem
        .window("Mandelbrot set visualization", WIDTH, HEIGHT)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
}

fn setup_gl_context(window: &sdl2::video::Window, video_subsystem: &VideoSubsystem) -> GLContext {
    let gl_context = window.gl_create_context().map_err(|e| e.to_string())
        .expect("Failed to create opengl context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    window.gl_make_current(&gl_context).map_err(|e| e.to_string())
        .expect("Failed to switch current opengl context");
    window.subsystem().gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
        .map_err(|e| e.to_string()).expect("Failed to setup VSync");

    gl_context
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

fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}