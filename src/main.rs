#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate glutin;
extern crate image;

mod shaders;
mod window;

fn main() {
  window::GlutinWindow {}.run();
}
