use glutin::{Event, WindowEvent, ControlFlow};
use gleam::gl;

fn run_window(title: &str, width: f64, height: f64) {
    // Tell unix to use x11 because wayland support is bad
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    // Lets create out event loop
    let mut event_loop = glutin::EventsLoop::new();

    // Define the window we want to build
    let wb = glutin::WindowBuilder::new()
        .with_title(&title.to_string())
        .with_dimensions(glutin::dpi::LogicalSize::new(width,height));
    
    // Define the opengl we want and then build it
    let windowed_context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles {
                    opengl_version: (3, 2),
                    opengles_version: (3, 0),
        })
        .build_windowed(wb, &event_loop).unwrap();
    
    // Take the opengl context
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    
    // Get a handle to opengl
    let gl = match windowed_context.get_api() {
            glutin::Api::OpenGl => unsafe {
                gl::GlFns::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _)
            },
            glutin::Api::OpenGlEs => unsafe {
                gl::GlesFns::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _)
            },
            glutin::Api::WebGl => unimplemented!(),
    };

    // Get a DPI factor
    let _device_pixel_ratio = windowed_context.window().get_hidpi_factor() as f32;
    
    // Render until closed
     event_loop.run_forever(|event| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
               ControlFlow::Break
            },
            // Other window events can be caught in here
            _ => {
                gl.clear_color(1.0,0.0,0.0,1.0);
                gl.clear(gl::COLOR_BUFFER_BIT);
                windowed_context.swap_buffers().ok();
                ControlFlow::Continue
            },
        }
    });
}



fn main() {
    run_window("Hello World",600.0,400.0);
}