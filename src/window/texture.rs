use gfx;
use gfx::{format::Rgba8, texture};
use image;
use std::io::Cursor;

pub struct CubemapData<'a> {
  pub up: &'a [u8],
  pub down: &'a [u8],
  pub front: &'a [u8],
  pub back: &'a [u8],
  pub right: &'a [u8],
  pub left: &'a [u8],
}

impl<'a> CubemapData<'a> {
  fn as_array(self) -> [&'a [u8]; 6] {
    [self.right, self.left, self.up, self.down, self.front, self.back]
  }
}

pub fn load_cubemap<R, F>(factory: &mut F, data: CubemapData) -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
                      where R: gfx::Resources, F: gfx::Factory<R> {
  let images = data.as_array().iter().map(|data| {
    image::load(Cursor::new(data), image::JPEG).unwrap().to_rgba()
  }).collect::<Vec<_>>();
  let data: [&[u8]; 6] = [&images[0], &images[1], &images[2], &images[3], &images[4], &images[5]];
  let kind = texture::Kind::Cube(images[0].dimensions().0 as u16);
  match factory.create_texture_immutable_u8::<Rgba8>(kind, texture::Mipmap::Provided, &data) {
    Ok((_, view)) => Ok(view),
    Err(_) => Err("Unable to create an immutable cubemap texture".to_owned()),
  }
}