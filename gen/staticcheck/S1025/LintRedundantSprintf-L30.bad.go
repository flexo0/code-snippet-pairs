package p

func fn() {
	_ = fmt.Sprintf("%s", t12) //@ diag(`underlying type is a slice of bytes`)
	_ = fmt.Sprintf("%s", t13) //@ diag(`underlying type is a slice of bytes`)
	_ = fmt.Sprintf("%s", t14) //@ diag(`underlying type is a slice of bytes`)
	_ = fmt.Sprintf("%s", t15) //@ diag(`is already a string`)
	_ = fmt.Sprintf("%s", t17) //@ diag(`should use String() instead of fmt.Sprintf`)
}
