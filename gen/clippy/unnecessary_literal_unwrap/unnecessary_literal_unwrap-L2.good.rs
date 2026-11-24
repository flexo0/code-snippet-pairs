#![allow(
    clippy::diverging_sub_expression,
    clippy::let_unit_value,
    clippy::no_effect,
    clippy::unnecessary_lazy_evaluations
)]

fn unwrap_option_some() {
    let _val = 1;
    //~^ unnecessary_literal_unwrap
    let _val = 1;
    //~^ unnecessary_literal_unwrap

    1;
    //~^ unnecessary_literal_unwrap
    1;
    //~^ unnecessary_literal_unwrap
}
