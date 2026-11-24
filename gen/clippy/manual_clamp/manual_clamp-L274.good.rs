fn const_main() {
    {
        let input: i32 = cmp_min_max(1);
        // These can only be detected if exactly one of the arguments to the inner function is const.
        let x16 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x17 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x18 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x19 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x20 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x21 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x22 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let x23 = input.clamp(CONST_MIN, CONST_MAX);
        //~^ manual_clamp

        let input: f64 = cmp_min_max(1) as f64;
        let x24 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x25 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x26 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x27 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x28 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x29 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x30 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp

        let x31 = input.clamp(CONST_F64_MIN, CONST_F64_MAX);
        //~^ manual_clamp
    }
}
