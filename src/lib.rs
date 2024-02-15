//! Diesel support for Point types in Postgres

#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod sql_types;
pub mod types;
