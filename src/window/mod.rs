use std;
use std::time::Instant;

use gfx;
use gfx::Device;
use gfx::memory::Typed;
use gfx::traits::FactoryExt;
use gfx_core::format::SurfaceType;
use glutin;
use glutin::dpi::LogicalSize;

use crate::shaders::{pipeline_data, Time, VertexData};
use crate::window::texture::CubemapData;

mod texture;

const RESOLUTION_X: u32 = 1200;
const RESOLUTION_Y: u32 = 900;

pub const COLOR_FORMAT_VALUE: SurfaceType = SurfaceType::R8_G8_B8_A8;
pub const DEPTH_FORMAT_VALUE: SurfaceType = SurfaceType::D24_S8;

const SHADER_VERT: &[u8] = include_bytes!("../shaders/tracer.v.glsl");
const SHADER_FRAG: &[u8] = include_bytes!("../shaders/tracer.f.glsl");

pub struct GlutinWindow {}

impl GlutinWindow {
  pub fn run(&mut self) {
    let vertex_data: [VertexData; 3] = [
      VertexData::new([-1.0, -1.0]),
      VertexData::new([3.0, -1.0]),
      VertexData::new([-1.0, 3.0])
    ];

    let mut events_loop = glutin::EventsLoop::new();

    let builder = glutin::WindowBuilder::new()
      .with_title("Raytracer")
      .with_dimensions(LogicalSize::new(RESOLUTION_X.into(), RESOLUTION_Y.into()));

    let window_context = glutin::ContextBuilder::new()
      .with_vsync(true)
      .with_double_buffer(Some(true))
      .with_pixel_format(24, 8)
      .with_srgb(true)
      .build_windowed(builder, &events_loop)
      .expect("Window context creation failed");

//    let window = glutin::Window::new(builder, window_context, &events_loop)
//      .expect("GLWindow creation failed");

    let (width, height) = {
      let inner_size = window_context.window().get_inner_size().expect("get_inner_size failed");
      let size = inner_size.to_physical(window_context.window().get_hidpi_factor());
      (size.width as _, size.height as _)
    };

    let window_context = unsafe {
      window_context
        .make_current()
        .expect("Window focus failed")
    };

    let (mut device, mut factory) = gfx_device_gl::create(|s|
      window_context.get_proc_address(s) as *const std::os::raw::c_void);

    let aa = window_context
      .get_pixel_format().multisampling
      .unwrap_or(0) as u8;

    let window_dimensions = (width, height, 1, aa.into());

    let (rtv, dsv) =
      gfx_device_gl::create_main_targets_raw(window_dimensions,
                                             COLOR_FORMAT_VALUE,
                                             DEPTH_FORMAT_VALUE);

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
      rtv: gfx::handle::RenderTargetView::new(rtv),
      dsv: gfx::handle::DepthStencilView::new(dsv),
    };

    loop {
      let elapsed = start_time.elapsed();
      let time = (f64::from(elapsed.subsec_nanos()) / 1e9 + elapsed.as_secs() as f64) as f32;

      encoder.clear_depth(&data.dsv, 1.0);
      encoder.update_constant_buffer(&data.time, &Time::new(time));
      encoder.draw(&slice, &pso, &data);

      window_context.swap_buffers().unwrap();
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
