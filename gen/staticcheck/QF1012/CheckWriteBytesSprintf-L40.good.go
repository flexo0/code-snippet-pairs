package p

func fn3() {
	fmt.Fprint(&buf, "abc", "de") //@ diag(`Use fmt.Fprint`)
}
