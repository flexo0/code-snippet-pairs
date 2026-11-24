package p

func fn1() {
	fmt.Fprint(w, "abc", "de")   //@ diag(`Use fmt.Fprint`)
	fmt.Fprintf(w, "%T", w)      //@ diag(`Use fmt.Fprintf`)
	fmt.Fprintln(w, "abc", "de") //@ diag(`Use fmt.Fprintln`)
}
