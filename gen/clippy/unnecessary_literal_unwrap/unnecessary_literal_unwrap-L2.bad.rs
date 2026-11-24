#![allow(
    clippy::diverging_sub_expression,
    clippy::let_unit_value,
    clippy::no_effect,
    clippy::unnecessary_lazy_evaluations
)]

fn unwrap_option_some() {
    let _val = Some(1).unwrap();
    //~^ unnecessary_literal_unwrap
    let _val = Some(1).expect("this never happens");
    //~^ unnecessary_literal_unwrap

    Some(1).unwrap();
    //~^ unnecessary_literal_unwrap
    Some(1).expect("this never happens");
    //~^ unnecessary_literal_unwrap
}
