#!/usr/bin/env rustx

fn hi () {
   io::println(~"Hello!");
}

pub fn main () {
   hi();
}

#[test]
fn test () {
   assert! (2 + 2 == 4);
}
