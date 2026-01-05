// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
use zephyr::cbs;

use zephyr::time::{sleep, Duration};
use zephyr::printkln;
#[no_mangle]
extern "C" fn rust_main() {
    printkln!("Starting CBS test");

    let mut my_cbs = cbs::cbs::cbs::new(5, b"cbs_thread", 1000, 2000);
    my_cbs.start();

    loop {
        // Main thread logic goes here
        printkln!("Main thread is running");
        sleep(Duration::from_ticks(500));
    }
 
}
