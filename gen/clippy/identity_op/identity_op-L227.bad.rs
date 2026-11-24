fn issue_12050() {
    {
        // this is just silly
        let x = &&&0i32;
        let _: i32 = ***x + 0;
        //~^ identity_op

        let _: i32 = **x + 0;
        //~^ identity_op

        let x = 0i32;
        let _: i32 = *&x + 0;
        //~^ identity_op

        let _: i32 = **&&x + 0;
        //~^ identity_op

        let _: i32 = *&*&x + 0;
        //~^ identity_op

        let _: i32 = **&&*&x + 0;
        //~^ identity_op
    }
}
