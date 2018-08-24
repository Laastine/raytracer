use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use glutin::GlContext;
use shaders::{pipeline_data, Time, VertexData};
use std;
use std::time::Instant;
use window::texture::CubemapData;

mod texture;

const RESOLUTION_X: u32 = 1200;
const RESOLUTION_Y: u32 = 900;

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
    let vertex_data: [VertexData; 3] = [
      VertexData::new([-1.0, -1.0]),
      VertexData::new([ 3.0, -1.0]),
      VertexData::new([-1.0,  3.0])
    ];

    let mut events_loop = glutin::EventsLoop::new();

    let window_builder = glutin::WindowBuilder::new()
      .with_title("Raytracer")
      .with_dimensions(RESOLUTION_X, RESOLUTION_Y);

    let context = glutin::ContextBuilder::new()
      .with_vsync(true)
      .with_pixel_format(24, 8)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengles_version: (3, 0),
        opengl_version: (3, 3),
      });

    let (window, mut device, mut factory, rtv, dsv) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(&SHADER_VERT, &SHADER_FRAG, pipeline_data::new())
                     .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, ());

    let cube_texture = texture::load_cubemap(&mut factory, &CubemapData {
      up: &include_bytes!("../../assets/clouds_up.jpg")[..],
      down: &include_bytes!("../../assets/clouds_down.jpg")[..],
      front: &include_bytes!("../../assets/clouds_north.jpg")[..],
      back: &include_bytes!("../../assets/clouds_south.jpg")[..],
      right: &include_bytes!("../../assets/clouds_east.jpg")[..],
      left: &include_bytes!("../../assets/clouds_west.jpg")[..],
    }).unwrap();

    let start_time = Instant::now();

    let data = pipeline_data::Data {
      vbuf: vertex_buffer,
      time: factory.create_constant_buffer(1),
      cube_texture: (cube_texture, factory.create_sampler_linear()),
      rtv,
      dsv
    };

    loop {
      let elapsed = start_time.elapsed();
      let time = (f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64) as f32;

      encoder.clear_depth(&data.dsv, 1.0);
      encoder.update_constant_buffer(&data.time, &Time::new(time));
      encoder.draw(&slice, &pso, &data);

      window.swap_buffers().unwrap();
      encoder.flush(&mut device);
      device.cleanup();

      events_loop.poll_events(|event| {
        use glutin::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        if let Event::WindowEvent { event, .. } = event {
          match event {
            WindowEvent::CloseRequested |
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