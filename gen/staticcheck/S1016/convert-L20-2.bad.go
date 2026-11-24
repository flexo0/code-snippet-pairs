package p

func fn() {
	_ = t2{v1.a, v1.b} //@ diag(`should convert v1`)
	_ = t3{v1.a, v1.b} //@ diag(`should convert v1`)
}
