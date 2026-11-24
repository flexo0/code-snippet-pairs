def okay_steps():
    text = "!x!y!z"
    if text.startswith("!"):
        text = text[1::1]
    if text.startswith("!"):
        text = text[1::True]
    if text.startswith("!"):
        text = text[1::None]
    print(text)


# this should be skipped
