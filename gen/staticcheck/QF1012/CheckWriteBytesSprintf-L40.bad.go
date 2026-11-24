package p

func fn3() {
	buf.WriteString(fmt.Sprint("abc", "de")) //@ diag(`Use fmt.Fprint`)
}
