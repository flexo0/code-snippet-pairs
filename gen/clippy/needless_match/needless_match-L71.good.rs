fn result_match() {
    let _: Result<i32, i32> = Ok(1);
    let _: Result<i32, i32> = func_ret_err(0_i32);
}
