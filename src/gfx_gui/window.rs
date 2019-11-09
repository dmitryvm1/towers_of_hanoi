use super::context;
use gfx_text::Renderer;

pub struct Window {
    pub context: context::Context<gfx_device_gl::Device, gfx_device_gl::Factory>,
    pub background: [f32; 4],
    pub window: glutin::WindowedContext,
    pub text: Renderer<gfx_device_gl::Resources, gfx_device_gl::Factory>,
    pub event_loop: glutin::EventsLoop
}

impl Window {
    pub fn new(title: &str) -> Window {
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_string());

        let event_loop = glutin::EventsLoop::new();

        let context_builder = glutin::ContextBuilder::new().with_vsync(true);
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<context::ColorFormat, context::DepthFormat>(builder, context_builder, &event_loop)
                .expect("Failed to initialize graphics");
        let text = gfx_text::new(factory.clone())
            .with_font("assets/UbuntuMono-R.ttf")
            .with_size(18)
            .build()
            .unwrap();
        let cbuf = factory.create_command_buffer();
        Window {
            text,
            event_loop,
            window: window,
            context: context::Context::new(device, factory, cbuf, main_color, main_depth),
            background: [0.0, 0.0, 0.0, 1.0],
        }
    }
}