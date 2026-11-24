fn issue14440(opt: Option<i32>) {
    opt.filter(|&x| unsafe { f(x as u32) });
}
