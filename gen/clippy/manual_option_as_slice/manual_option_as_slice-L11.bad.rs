fn check(x: Option<u32>) {
    _ = if let Some(f) = x.as_ref() {
        //~^ manual_option_as_slice

        std::slice::from_ref(f)
    } else {
        &[]
    };

    _ = x.as_ref().map_or(&[][..], std::slice::from_ref);
}
