fn main() {
    {
        enum AnEnum {
            X,
            Y,
        }

        fn foo(_x: AnEnum) {}

        fn main() {
            let z = AnEnum::X;
            // we can't remove the reference here!
            let _ = matches!(&z, AnEnum::X);
            //~^^^^ match_like_matches_macro
            foo(z);
        }
    }
}
