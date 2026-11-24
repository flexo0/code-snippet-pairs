package p

func fn() {
	_ = fmt.Sprintf("%s", "test")      //@ diag(`is already a string`)
	_ = fmt.Sprintf("%s", t1)          //@ diag(`is a string`)
	_ = fmt.Sprintf("%s", t2)          //@ diag(`is a string`)
	_ = fmt.Sprintf("%s", t3)          //@ diag(`should use String() instead of fmt.Sprintf`)
	_ = fmt.Sprintf("%s", t3.String()) //@ diag(`is already a string`)
}
