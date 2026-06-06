#![cfg(unix)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
// FIXME I guess
#![allow(clippy::all)]
#![warn(clippy::absurd_extreme_comparisons)]
#![warn(clippy::eq_op)]
#![warn(clippy::unnecessary_cast)]
#![warn(clippy::field_reassign_with_default)]
#![warn(clippy::manual_clamp)]
#![warn(clippy::manual_checked_ops)]
#![warn(clippy::double_parens)]
#![warn(clippy::redundant_field_names)]
#![warn(clippy::if_same_then_else)]
extern crate libc;

pub mod benchfn;
pub mod benchzstd;
pub mod datagen;
pub mod dibio;
pub mod fileio;
pub mod fileio_asyncio;
pub mod lorem;
pub mod timefn;
pub mod util;
pub mod zstdcli;
pub mod zstdcli_trace;

fn main() {}
