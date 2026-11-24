def remove_extension_via_slice(filename: str) -> str:
    if filename.endswith(".txt"):
        filename = filename[:-4]
