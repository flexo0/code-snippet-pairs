package p

func fn() {
	_ = bytes.IndexRune(nil, 'x') > -1 //@ diag(` bytes.ContainsRune`)
	_ = bytes.IndexAny(nil, "") > -1   //@ diag(` bytes.ContainsAny`)
	_ = bytes.Index(nil, nil) > -1     //@ diag(` bytes.Contains`)
}
