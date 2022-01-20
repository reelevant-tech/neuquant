#![deny(clippy::all)]

use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct NeuQuantResult {
  pub indixes: Uint8Array,
  pub color_map: Uint8Array
}

#[napi]
pub fn neuquant(samplefac: i32, colors: u32, data: Uint8Array) -> NeuQuantResult {
  let pixels: Vec<u8> = data.to_vec();
  let colors: usize = colors.try_into().unwrap();

  let nq = color_quant::NeuQuant::new(samplefac, colors, &pixels);
  let indixes: Vec<u8> = pixels.chunks(4).map(|pix| nq.index_of(pix) as u8).collect();
  let color_map = nq.color_map_rgba();

  NeuQuantResult {
    indixes: Uint8Array::new(indixes),
    color_map: Uint8Array::new(color_map)
  }
}

