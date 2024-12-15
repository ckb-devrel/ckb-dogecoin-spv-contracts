//! Utilities for tests only.

use env_logger::{Builder, Target};
use log::LevelFilter;

mod data_helper;
mod type_id;

pub(crate) use type_id::calculate_type_id;

pub(crate) fn setup() {
    let _ = Builder::new()
        .filter_module("tests", LevelFilter::Trace)
        .filter_module("ckb_bitcoin_spv", LevelFilter::Trace)
        .target(Target::Stdout)
        .is_test(true)
        .try_init();
    println!();
}

pub(crate) fn _prev_client_id(current: u8, count: u8) -> u8 {
    if current == 0 {
        count - 1
    } else {
        current - 1
    }
}

pub(crate) fn _next_client_id(current: u8, count: u8) -> u8 {
    if current + 1 < count {
        current + 1
    } else {
        0
    }
}
