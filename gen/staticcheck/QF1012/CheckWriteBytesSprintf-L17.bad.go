package p

func fn1() {
	w.Write([]byte(fmt.Sprint("abc", "de")))   //@ diag(`Use fmt.Fprint`)
	w.Write([]byte(fmt.Sprintf("%T", w)))      //@ diag(`Use fmt.Fprintf`)
	w.Write([]byte(fmt.Sprintln("abc", "de"))) //@ diag(`Use fmt.Fprintln`)
}
