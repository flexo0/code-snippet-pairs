package p

func fn() {
	if true {
		var x string //@ diag(`should merge variable declaration with assignment on next line`)
		x = ""
		_ = x
	}
}
