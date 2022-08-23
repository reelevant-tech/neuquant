#![deny(clippy::all)]

use napi::bindgen_prelude::*;
mod neuquant;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct NeuQuantResult {
  pub indixes: Uint8Array,
  pub color_map: Uint8Array,
}

#[napi]
pub fn neuquant(samplefac: i32, colors: u32, data: Uint8Array) -> AsyncTask<AsyncNeuQuant> {
  AsyncTask::new(AsyncNeuQuant {
    samplefac,
    colors: colors.try_into().unwrap(),
    pixels: data.to_vec(),
  })
}

pub struct AsyncNeuQuant {
  samplefac: i32,
  colors: usize,
  pixels: Vec<u8>,
}

pub struct NeuQuantOutput {
  indixes: Vec<u8>,
  color_map: Vec<u8>,
}

#[napi]
impl Task for AsyncNeuQuant {
  type Output = NeuQuantOutput;
  type JsValue = NeuQuantResult;

  fn compute(&mut self) -> Result<Self::Output> {
    let nq = neuquant::NeuQuant::new(self.samplefac, self.colors, &self.pixels);
    let indixes: Vec<u8> = self
      .pixels
      .chunks(4)
      .map(|pix| nq.index_of(pix) as u8)
      .collect();
    let color_map = nq.color_map_rgba();

    Ok(NeuQuantOutput { indixes, color_map })
  }

  fn resolve(&mut self, _env: napi::Env, output: NeuQuantOutput) -> Result<Self::JsValue> {
    Ok(NeuQuantResult {
      indixes: Uint8Array::new(output.indixes),
      color_map: Uint8Array::new(output.color_map),
    })
  }
}
