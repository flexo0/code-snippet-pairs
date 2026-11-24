package p

func fn() {
	http.Error(nil, "", 506)         //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.Redirect(nil, nil, "", 506) //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.StatusText(506)             //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.RedirectHandler("", 506)    //@ diag(`http.StatusVariantAlsoNegotiates`)
}
