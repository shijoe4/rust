// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]
use zephyr::cbs;

use zephyr::time::{sleep, Duration};
use zephyr::printkln;
#[no_mangle]
extern "C" fn rust_main() {
    printkln!("Starting CBS test");

    let mut my_cbs1 = cbs::cbs::cbs::new(5, b"cbs_thread 1", 1000, 2000);
    server_handle1 =my_cbs1.start();



    let mut my_cbs2 = cbs::cbs::cbs::new(5, b"cbs_thread 2", 1000, 2000);
    let server_handle2  my_cbs2.start();

    loop {
        // Main thread logic goes here
        printkln!("Main thread is running");
        sleep(Duration::from_ticks(500));

        my_cbs1.push(|| {
            add_x(10, 20);
        });

        my_cbs2.push(|| {
            add_xf(10.5, 20.3);
        });

    }
 

    server_handle1.join().unwrap();
    server_handle2.join().unwrap();
}



fn add_x(a: i32, b: i32)
{
    let c = a + b;
    printkln!("Sum is: {}", c);
}

fn add_xf(a: f32, b: f32)
{
    let c = a + b;
    printkln!("Sum float: {}", c);
}