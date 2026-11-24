package p

func foo() {
	_ = Alias(v1) //@ diag(`should convert v1`)
	_ = S1(v2)    //@ diag(`should convert v2`)
}
