fn not_ok() {
    {
        foo_mrslice(mrslice.as_mut());
        //~^ useless_asref
        foo_mrslice(mrslice);
        foo_rslice(mrslice.as_ref());
        //~^ useless_asref
        foo_rslice(mrslice);
    }
}
