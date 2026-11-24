package p

func fn() {
	_ = fmt.Errorf("%d", 0) //@ diag(`should use fmt.Errorf`)
}
