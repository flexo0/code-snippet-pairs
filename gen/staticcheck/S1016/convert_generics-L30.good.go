package p

func _() {
	_ = T4[int](t1) //@ diag(`(type T1) to T4[int]`)
	_ = T4[int](t3) //@ diag(`(type T3[int]) to T4[int]`)
}
