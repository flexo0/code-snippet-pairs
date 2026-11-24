package p

func fn() {
	_ = []AliasByte(m["key"].String()) //@ diag(`should use m["key"].Bytes() instead of []AliasByte(m["key"].String())`)
	_ = AliasSlice(m["key"].String())  //@ diag(`should use m["key"].Bytes() instead of AliasSlice(m["key"].String())`)
	_ = AliasSlice2(m["key"].String()) //@ diag(`should use m["key"].Bytes() instead of AliasSlice2(m["key"].String())`)
}
