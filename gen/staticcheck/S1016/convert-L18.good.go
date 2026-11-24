package p

func fn() {
	_ = t2(v1) //@ diag(`should convert v1`)
	_ = t2(v1) //@ diag(`should convert v1`)
	_ = t2(v1) //@ diag(`should convert v1`)
	_ = t3(v1) //@ diag(`should convert v1`)
}
