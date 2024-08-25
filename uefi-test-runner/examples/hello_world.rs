// ANCHOR: all
// ANCHOR: features
#![no_main]
#![no_std]
// ANCHOR_END: features

// ANCHOR: use
use log::info;
use uefi::boot;
use uefi::prelude::*;
// ANCHOR_END: use

// ANCHOR: entry
#[entry]
fn main() -> Status {
    // ANCHOR_END: entry
    // ANCHOR: services
    uefi::helpers::init().unwrap();
    // ANCHOR_END: services
    // ANCHOR: log
    info!("Hello world!");
    boot::stall(10_000_000);
    // ANCHOR_END: log
    // ANCHOR: return
    Status::SUCCESS
}
// ANCHOR_END: return
// ANCHOR_END: all
