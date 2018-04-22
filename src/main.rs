#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

mod shaders;
mod window;

fn main() {
  let mut gl_window = window::GlutinWindow::new();
  gl_window.run();
}
