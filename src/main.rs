#![feature(asm)]


#[macro_use]
#[no_link]
extern crate probe;

use std::time::Duration;
use std::thread::sleep;


fn main() {
    probe!(foo, begin);
    let mut total = 0;
    for i in 0..1000 {
        total += i;
        sleep(Duration::from_millis(1000));
        probe!(foo, loop, i, total);
        println!("{}", i);
    }
    assert_eq!(total, 4950);
    probe!(foo, end);
}


