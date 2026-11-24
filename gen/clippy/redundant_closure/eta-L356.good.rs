pub fn mutable_impl_fn_mut(mut f: impl FnMut(), mut f_used_once: impl FnMut()) -> impl FnMut() {
    move || takes_fn_mut(&mut f_used_once)
    //~^ redundant_closure
}
