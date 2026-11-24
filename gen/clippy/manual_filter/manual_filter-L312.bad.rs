fn issue14440(opt: Option<i32>) {
    opt.and_then(|x| if unsafe { f(x as u32) } { Some(x) } else { None });
}
