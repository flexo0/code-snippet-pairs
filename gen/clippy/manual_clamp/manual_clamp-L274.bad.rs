fn const_main() {
    {
        let input: i32 = cmp_min_max(1);
        // These can only be detected if exactly one of the arguments to the inner function is const.
        let x16 = cmp_max(cmp_min(input, CONST_MAX), CONST_MIN);
        //~^ manual_clamp

        let x17 = cmp_min(cmp_max(input, CONST_MIN), CONST_MAX);
        //~^ manual_clamp

        let x18 = cmp_max(CONST_MIN, cmp_min(input, CONST_MAX));
        //~^ manual_clamp

        let x19 = cmp_min(CONST_MAX, cmp_max(input, CONST_MIN));
        //~^ manual_clamp

        let x20 = cmp_max(cmp_min(CONST_MAX, input), CONST_MIN);
        //~^ manual_clamp

        let x21 = cmp_min(cmp_max(CONST_MIN, input), CONST_MAX);
        //~^ manual_clamp

        let x22 = cmp_max(CONST_MIN, cmp_min(CONST_MAX, input));
        //~^ manual_clamp

        let x23 = cmp_min(CONST_MAX, cmp_max(CONST_MIN, input));
        //~^ manual_clamp

        let input: f64 = cmp_min_max(1) as f64;
        let x24 = f64::max(f64::min(input, CONST_F64_MAX), CONST_F64_MIN);
        //~^ manual_clamp

        let x25 = f64::min(f64::max(input, CONST_F64_MIN), CONST_F64_MAX);
        //~^ manual_clamp

        let x26 = f64::max(CONST_F64_MIN, f64::min(input, CONST_F64_MAX));
        //~^ manual_clamp

        let x27 = f64::min(CONST_F64_MAX, f64::max(input, CONST_F64_MIN));
        //~^ manual_clamp

        let x28 = f64::max(f64::min(CONST_F64_MAX, input), CONST_F64_MIN);
        //~^ manual_clamp

        let x29 = f64::min(f64::max(CONST_F64_MIN, input), CONST_F64_MAX);
        //~^ manual_clamp

        let x30 = f64::max(CONST_F64_MIN, f64::min(CONST_F64_MAX, input));
        //~^ manual_clamp

        let x31 = f64::min(CONST_F64_MAX, f64::max(CONST_F64_MIN, input));
        //~^ manual_clamp
    }
}
