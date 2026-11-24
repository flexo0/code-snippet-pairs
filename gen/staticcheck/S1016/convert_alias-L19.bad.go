package p

func foo() {
	_ = Alias{A: v1.A} //@ diag(`should convert v1`)
	_ = S1{A: v2.A}    //@ diag(`should convert v2`)
}
