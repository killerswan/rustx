extern crate extra;

use std::io;

#[main]
fn say_hi () {
   io::println("Hello!!");
}

#[test]
fn addition_works () {
   assert! (2 + 2 == 4);
}

#[bench]
fn addition_benchmarked (b: &mut extra::test::BenchHarness) {
   let mut sum = 0;
   b.iter(|| sum += 1)
}
