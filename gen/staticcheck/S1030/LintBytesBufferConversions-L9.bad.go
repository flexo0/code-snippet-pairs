package p

func fn() {
	_ = string(buf.Bytes())  //@ diag(`should use buf.String() instead of string(buf.Bytes())`)
	_ = []byte(buf.String()) //@ diag(`should use buf.Bytes() instead of []byte(buf.String())`)
}
