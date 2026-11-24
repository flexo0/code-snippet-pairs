package p

func fn() {
	var x int //@ diag(`should merge variable declaration with assignment on next line`)
	x = 1
}
