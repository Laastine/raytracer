use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use glutin::GlContext;
use shaders::{pipeline_data};

const RESOLUTION_X: u32 = 1280;
const RESOLUTION_Y: u32 = 720;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/tracer.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/tracer.f.glsl");

pub struct GlutinWindow {
}

impl GlutinWindow {
  pub fn new() -> GlutinWindow {
    GlutinWindow {
    }
  }

  pub fn run(&mut self) {
    let mut events_loop = glutin::EventsLoop::new();

    let window_title = glutin::WindowBuilder::new()
      .with_title("Raytracer");

    let window_builder = window_title.with_dimensions(RESOLUTION_X, RESOLUTION_Y);

    let context = glutin::ContextBuilder::new()
      .with_vsync(true)
      .with_pixel_format(24, 8)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengles_version: (3, 0),
        opengl_version: (3, 3),
      });

    let (window, mut device, mut factory, rtv, dsv) = gfx_window_glutin::init::<ColorFormat,
      DepthFormat>(window_builder, context, &events_loop);

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(&SHADER_VERT, &SHADER_FRAG, pipeline_data::new())
                     .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&[], ());

    let data = pipeline_data::Data {
      vbuf: vertex_buffer,
      rtv,
      dsv
    };

    events_loop.run_forever(move |event| {
      use glutin::{ControlFlow, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

      if let Event::WindowEvent { event, .. } = event {
        match event {
          WindowEvent::Closed |
          WindowEvent::KeyboardInput {
            input: KeyboardInput {
              virtual_keycode: Some(VirtualKeyCode::Escape),
              ..
            },
            ..
          } => return ControlFlow::Break,
          _ => (),
        }
      }

      encoder.clear(&data.rtv, [0.2, 0.2, 0.2, 1.0]);
      encoder.clear_depth(&data.dsv, 1.0);
      encoder.draw(&slice, &pso, &data);
      encoder.flush(&mut device);
      window.swap_buffers().unwrap();
      device.cleanup();

      ControlFlow::Continue
    });
  }
}