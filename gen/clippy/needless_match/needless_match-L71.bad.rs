fn result_match() {
    let _: Result<i32, i32> = match Ok(1) {
        //~^ needless_match
        Ok(a) => Ok(a),
        Err(err) => Err(err),
    };
    let _: Result<i32, i32> = match func_ret_err(0_i32) {
        //~^ needless_match
        Err(err) => Err(err),
        Ok(a) => Ok(a),
    };
}
