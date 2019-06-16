//! # roctokit
//!
//! roctokit is a rust client for the Github API (v3) to make interrogating github data a breeze in rust

extern crate futures;
extern crate serde_json;
extern crate serde;
extern crate reqwest;
extern crate chrono;

pub mod clients;

#[cfg(test)]
mod tests;
