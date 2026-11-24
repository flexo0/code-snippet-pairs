package p

func fn() {
	_ = "foo" //@ diag(`unnecessary use of fmt.Sprint`)
	_ = "foo" //@ diag(`unnecessary use of fmt.Sprintf`)
}
