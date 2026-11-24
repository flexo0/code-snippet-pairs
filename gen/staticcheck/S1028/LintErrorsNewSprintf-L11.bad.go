package p

func fn() {
	_ = errors.New(fmt.Sprintf("%d", 0)) //@ diag(`should use fmt.Errorf`)
}
