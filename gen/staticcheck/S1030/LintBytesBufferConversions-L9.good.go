package p

func fn() {
	_ = buf.String() //@ diag(`should use buf.String() instead of string(buf.Bytes())`)
	_ = buf.Bytes()  //@ diag(`should use buf.Bytes() instead of []byte(buf.String())`)
}
