package p

func fn() {
	_ = strings.Contains("", "") //@ diag(` strings.Contains`)
	_ = strings.Contains("", "") //@ diag(` strings.Contains`)
}
