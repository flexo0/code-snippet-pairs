#![allow(
    clippy::unit_arg,
    clippy::unnecessary_lazy_evaluations,
    clippy::unused_unit,
    clippy::unwrap_or_default
)]

fn main() {
    if true { "a" } else { "b" };
    //~^ obfuscated_if_else

    if true { "a" } else { "b" };
    //~^ obfuscated_if_else

    let a = 1;
    if a == 1 { "a" } else { "b" };
    //~^ obfuscated_if_else

    if a == 1 { "a" } else { "b" };
    //~^ obfuscated_if_else

    let partial = (a == 1).then_some("a");
    partial.unwrap_or("b"); // not lint

    let mut a = 0;
    if true { a += 1 } else { () };
    //~^ obfuscated_if_else

    if true { () } else { a += 2 };
    //~^ obfuscated_if_else

    let mut n = 1;
    if true { n = 1 } else { n = 2 };
    //~^ obfuscated_if_else
    if true { 1 } else { n * 2 };
    //~^ obfuscated_if_else
    if true { n += 1 } else { () };
    //~^ obfuscated_if_else

    let _ = if true { 1 } else { n * 2 };
    //~^ obfuscated_if_else

    if true { 1 } else { Default::default() };
    //~^ obfuscated_if_else

    let partial = true.then_some(1);
    partial.unwrap_or_else(|| n * 2); // not lint

    if true { () } else { Default::default() };
    //~^ obfuscated_if_else

    if true { () } else { Default::default() };
    //~^ obfuscated_if_else

    if true { 1 } else { Default::default() };
    //~^ obfuscated_if_else

    if true { 1 } else { Default::default() };
    //~^ obfuscated_if_else
}
