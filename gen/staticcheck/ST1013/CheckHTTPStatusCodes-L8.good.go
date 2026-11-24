package p

func fn() {
	http.Error(nil, "", http.StatusVariantAlsoNegotiates)         //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.Redirect(nil, nil, "", http.StatusVariantAlsoNegotiates) //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.StatusText(http.StatusVariantAlsoNegotiates)             //@ diag(`http.StatusVariantAlsoNegotiates`)
	http.RedirectHandler("", http.StatusVariantAlsoNegotiates)    //@ diag(`http.StatusVariantAlsoNegotiates`)
}
