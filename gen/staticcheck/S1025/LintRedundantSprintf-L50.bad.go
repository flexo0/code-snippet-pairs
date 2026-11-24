package p

func fn() {
	_ = fmt.Sprintf("%s", t6) //@ diag(`should use String() instead of fmt.Sprintf`)
	_ = fmt.Sprintf("%s", t7) //@ diag(`underlying type is a slice of bytes`)
}
