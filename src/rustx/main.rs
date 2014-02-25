extern crate test;

use std::io;

#[main]
#[cfg(not(test))]
fn say_hi () {
   io::println("Hello!!");
}

#[test]
fn addition_works () {
   io::println("Hello!!");
   assert! (2 + 2 == 4);
}

#[bench]
fn addition_benchmarked (b: &mut test::BenchHarness) {
   let mut sum = 0;
   b.iter(|| sum += 1);
}
