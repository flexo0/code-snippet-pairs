fn not_ok() {
    {
        foo_mrslice(mrslice);
        //~^ useless_asref
        foo_mrslice(mrslice);
        foo_rslice(mrslice);
        //~^ useless_asref
        foo_rslice(mrslice);
    }
}
