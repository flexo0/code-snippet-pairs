package p

func fn() {
	_ = bytes.Equal(nil, nil)  //@ diag(` bytes.Equal`)
	_ = !bytes.Equal(nil, nil) //@ diag(`!bytes.Equal`)
}
