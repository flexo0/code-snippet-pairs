#![allow(
    clippy::unit_arg,
    clippy::unnecessary_lazy_evaluations,
    clippy::unused_unit,
    clippy::unwrap_or_default
)]

fn main() {
    true.then_some("a").unwrap_or("b");
    //~^ obfuscated_if_else

    true.then(|| "a").unwrap_or("b");
    //~^ obfuscated_if_else

    let a = 1;
    (a == 1).then_some("a").unwrap_or("b");
    //~^ obfuscated_if_else

    (a == 1).then(|| "a").unwrap_or("b");
    //~^ obfuscated_if_else

    let partial = (a == 1).then_some("a");
    partial.unwrap_or("b"); // not lint

    let mut a = 0;
    true.then_some(a += 1).unwrap_or(());
    //~^ obfuscated_if_else

    true.then_some(()).unwrap_or(a += 2);
    //~^ obfuscated_if_else

    let mut n = 1;
    true.then(|| n = 1).unwrap_or_else(|| n = 2);
    //~^ obfuscated_if_else
    true.then_some(1).unwrap_or_else(|| n * 2);
    //~^ obfuscated_if_else
    true.then_some(n += 1).unwrap_or_else(|| ());
    //~^ obfuscated_if_else

    let _ = true.then_some(1).unwrap_or_else(|| n * 2);
    //~^ obfuscated_if_else

    true.then_some(1).unwrap_or_else(Default::default);
    //~^ obfuscated_if_else

    let partial = true.then_some(1);
    partial.unwrap_or_else(|| n * 2); // not lint

    true.then_some(()).unwrap_or_default();
    //~^ obfuscated_if_else

    true.then(|| ()).unwrap_or_default();
    //~^ obfuscated_if_else

    true.then_some(1).unwrap_or_default();
    //~^ obfuscated_if_else

    true.then(|| 1).unwrap_or_default();
    //~^ obfuscated_if_else
}
