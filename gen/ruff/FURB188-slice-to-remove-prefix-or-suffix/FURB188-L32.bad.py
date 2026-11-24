def remove_prefix_via_len(filename: str, prefix: str) -> str:
    return filename[len(prefix):] if filename.startswith(prefix) else filename


# these should not
