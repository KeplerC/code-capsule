#![feature(test)]

extern crate core;
extern crate test;

use core::loader;

use test::Bencher;

#[bench]
fn bench_hazard(b: &mut Bencher) {
    b.iter(|| {
        hazard::run_worker().unwrap();
    })
}
