package p

func fn() {
	_ = fmt.Sprint("foo")  //@ diag(`unnecessary use of fmt.Sprint`)
	_ = fmt.Sprintf("foo") //@ diag(`unnecessary use of fmt.Sprintf`)
}
