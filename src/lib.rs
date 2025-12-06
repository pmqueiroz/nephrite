#![deny(clippy::all)]
use kernel::_plus_100;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  _plus_100(input)
}
