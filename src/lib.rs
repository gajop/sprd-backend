// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

pub mod db;
pub mod gz;
pub mod rapid;
pub mod rapid_store;
pub mod sdp;
