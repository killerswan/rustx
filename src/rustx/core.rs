#!/usr/bin/env rustx

extern mod extra;

#[main]
fn say_hi () {
   println("Hello!!");
}

#[test]
fn addition_works () {
   assert! (2 + 2 == 4);
}

#[bench]
fn addition_benchmarked (b: &mut extra::test::BenchHarness) {
   let mut sum = 0;
   do b.iter {
      sum += 1;
   }
}
