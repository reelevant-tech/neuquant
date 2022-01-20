#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct NeuQuantResult {
  pub indixes: Buffer,
  pub color_map: Buffer,
}

#[napi]
pub fn neuquant(samplefac: i32, colors: u32, data: Buffer) -> NeuQuantResult {
  let pixels: Vec<u8> = data.into();
  let colors: usize = colors.try_into().unwrap();

  let nq = color_quant::NeuQuant::new(samplefac, colors, &pixels);
  let indixes: Vec<u8> = pixels.chunks(4).map(|pix| nq.index_of(pix) as u8).collect();
  let color_map = nq.color_map_rgba();

  NeuQuantResult {
    indixes: Buffer::from(indixes),
    color_map: Buffer::from(color_map)
  }
}

