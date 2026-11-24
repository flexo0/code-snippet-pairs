package p

func example(str, prefix string) string {
	if strings.HasPrefix(str, prefix) {
		str = str[len(prefix):]
	}
	return str
}
