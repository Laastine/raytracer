use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use glutin::GlContext;
use shaders::{pipeline_data, Time, VertexData};
use std;
use std::time::Instant;

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
    let vertex_data: [VertexData; 4] = [
      VertexData::new([-1.0, -1.0], [0.0, 1.0]),
      VertexData::new([1.0, -1.0], [1.0, 1.0]),
      VertexData::new([1.0, 1.0], [1.0, 0.0]),
      VertexData::new([-1.0, 1.0], [0.0, 0.0]),
    ];

    let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];

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
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let start_time = Instant::now();

    let data = pipeline_data::Data {
      vbuf: vertex_buffer,
      time: factory.create_constant_buffer(1),
      rtv,
      dsv
    };

    loop {
      let elapsed = start_time.elapsed();
      let time = (f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64) as f32;

      encoder.clear(&data.rtv, [1.0, 1.0, 1.0, 1.0]);
      encoder.clear_depth(&data.dsv, 1.0);
      encoder.update_constant_buffer(&data.time, &Time::new(time));
      encoder.draw(&slice, &pso, &data);
      encoder.flush(&mut device);
      window.swap_buffers().unwrap();
      device.cleanup();

      events_loop.poll_events(|event| {
        use glutin::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        if let Event::WindowEvent { event, .. } = event {
          match event {
            WindowEvent::Closed |
            WindowEvent::KeyboardInput {
              input: KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
              },
              ..
            } => std::process::exit(0),
            _ => (),
          }
        }
      });
    }
  }
}