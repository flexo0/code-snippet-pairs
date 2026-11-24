package p

func fn() {
	_ = "test"      //@ diag(`is already a string`)
	_ = string(t1)  //@ diag(`is a string`)
	_ = string(t2)  //@ diag(`is a string`)
	_ = t3.String() //@ diag(`should use String() instead of fmt.Sprintf`)
	_ = t3.String() //@ diag(`is already a string`)
}
