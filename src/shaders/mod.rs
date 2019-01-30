use gfx;

gfx_defines! {
  vertex VertexData {
    pos: [f32; 2] = "a_Pos",
  }

  constant Time {
    time: f32 = "u_Time",
  }

  pipeline pipeline_data {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    time: gfx::ConstantBuffer<Time> = "b_Time",
    cube_texture: gfx::TextureSampler<[f32; 4]> = "t_Cubemap",
    rtv: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    dsv: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}

impl Time {
  pub fn new(time: f32) -> Time {
    Time {
      time
    }
  }
}

impl VertexData {
  pub fn new(pos: [f32; 2]) -> VertexData {
    VertexData {
      pos,
    }
  }
}
