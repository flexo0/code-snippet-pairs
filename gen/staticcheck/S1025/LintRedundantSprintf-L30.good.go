package p

func fn() {
	_ = string(t12)  //@ diag(`underlying type is a slice of bytes`)
	_ = string(t13)  //@ diag(`underlying type is a slice of bytes`)
	_ = string(t14)  //@ diag(`underlying type is a slice of bytes`)
	_ = t15          //@ diag(`is already a string`)
	_ = t17.String() //@ diag(`should use String() instead of fmt.Sprintf`)
}
