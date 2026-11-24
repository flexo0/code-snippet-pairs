package p

func fn() {
	_ = bytes.ContainsRune(nil, 'x') //@ diag(` bytes.ContainsRune`)
	_ = bytes.ContainsAny(nil, "")   //@ diag(` bytes.ContainsAny`)
	_ = bytes.Contains(nil, nil)     //@ diag(` bytes.Contains`)
}
