use gfx;

gfx_defines! {
  vertex VertexData {
    pos: [f32; 2] = "a_Pos",
    buf_pos: [f32; 2] = "a_BufPos",
  }

  pipeline pipeline_data {
    vbuf: gfx::VertexBuffer<VertexData> = (),
    rtv: gfx::RenderTarget<gfx::format::Rgba8> = "Target0",
    dsv: gfx::DepthTarget<gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}