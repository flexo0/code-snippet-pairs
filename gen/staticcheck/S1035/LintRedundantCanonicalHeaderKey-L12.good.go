package p

func fn1() {
	headers.Add("test", "test") //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Del("test")         //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Get("test")         //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Set("test", "test") //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
}
