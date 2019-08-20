//! # roctokit
//!
//! roctokit is a rust client for the Github API (v3) to make interrogating github data a breeze in rust
#[macro_use] extern crate lazy_static;

pub mod clients;

#[cfg(test)]
mod tests;
