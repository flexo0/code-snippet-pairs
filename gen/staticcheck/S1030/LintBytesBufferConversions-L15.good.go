package p

func fn() {
	_ = m["key"].Bytes() //@ diag(`should use m["key"].Bytes() instead of []AliasByte(m["key"].String())`)
	_ = m["key"].Bytes() //@ diag(`should use m["key"].Bytes() instead of AliasSlice(m["key"].String())`)
	_ = m["key"].Bytes() //@ diag(`should use m["key"].Bytes() instead of AliasSlice2(m["key"].String())`)
}
