package p

func example(str, prefix string) string {
	str = strings.TrimPrefix(str, prefix)
	return str
}
