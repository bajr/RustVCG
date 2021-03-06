#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
/*
fn main() {
    let mut x = 3;
    x = add_five(x);
    x = add_three(x);
    println!("{:?}", x);
}
*/
fn main() {}
// This is acceptable: it is placed over a function
#[condition(pre="x:i32 == 25i32", post="return:i32 == (x:i32 % 5i32)")]
fn add_five_or_three(x: i32) -> i32 {
    x % 4
}
