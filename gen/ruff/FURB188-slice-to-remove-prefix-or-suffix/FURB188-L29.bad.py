def remove_prefix(filename: str) -> str:
    return filename[4:] if filename.startswith("abc-") else filename
