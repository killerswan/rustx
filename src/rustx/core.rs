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
fn bench_sum_1024_ints(b: &mut extra::test::BenchHarness) {
   use std::vec;

   let v = vec::from_fn(1024, |n| n);
   do b.iter {
      v.iter().fold(0, |old, new| old + *new);
   }
}
