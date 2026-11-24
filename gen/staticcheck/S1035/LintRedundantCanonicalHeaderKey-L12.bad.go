package p

func fn1() {
	headers.Add(http.CanonicalHeaderKey("test"), "test") //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Del(http.CanonicalHeaderKey("test"))         //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Get(http.CanonicalHeaderKey("test"))         //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
	headers.Set(http.CanonicalHeaderKey("test"), "test") //@ diag(`calling net/http.CanonicalHeaderKey on the 'key' argument of`)
}
