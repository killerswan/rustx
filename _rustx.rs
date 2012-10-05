#!/usr/bin/env rustx

extern mod std;

fn main () {
   io::println("Hello!");
}

#[test]
fn test () {
   assert 2 + 2 == 4;
}

