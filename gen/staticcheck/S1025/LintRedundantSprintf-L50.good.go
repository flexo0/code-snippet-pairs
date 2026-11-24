package p

func fn() {
	_ = t6.String() //@ diag(`should use String() instead of fmt.Sprintf`)
	_ = string(t7)  //@ diag(`underlying type is a slice of bytes`)
}
