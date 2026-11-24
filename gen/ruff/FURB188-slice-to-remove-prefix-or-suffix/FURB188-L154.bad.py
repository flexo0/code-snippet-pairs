def shadow_builtins(filename: str, extension: str) -> None:
    return filename[:-builtins_len(extension)] if filename.endswith(extension) else filename
