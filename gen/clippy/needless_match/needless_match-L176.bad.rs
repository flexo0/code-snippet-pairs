mod issue8542 {
    fn match_test() {
        let _: Complex = match ce {
            //~^ needless_match
            Complex::A(a) => Complex::A(a),
            Complex::B(a, b) => Complex::B(a, b),
            Complex::C(a, b, c) => Complex::C(a, b, c),
            Complex::D(E::VariantA(ea, eb), b) => Complex::D(E::VariantA(ea, eb), b),
            Complex::D(E::VariantB(ea, eb), b) => Complex::D(E::VariantB(ea, eb), b),
        };
    }
}
