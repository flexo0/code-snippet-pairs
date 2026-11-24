fn parse_diff_tree(output: &[u8]) -> Result<Vec<Change>> {
    let mut fields = output.split(|&b| b == 0).filter(|f| !f.is_empty());
    let mut changes = Vec::new();
    while let Some(header) = fields.next() {
        let header = std::str::from_utf8(header).with_context(|| {
            format!(
                "unexpected `git diff-tree` output: `{}`",
                String::from_utf8_lossy(header)
            )
        })?;
        let parts: Vec<&str> = header.trim_start_matches(':').split(' ').collect();
        let [mode_before, mode_after, blob_before, blob_after, status] = parts[..] else {
            bail!("unexpected `git diff-tree` output: `{header}`");
        };
        let mut path = || {
            fields
                .next()
                .map(path_of)
                .with_context(|| format!("no path after `{header}` in `git diff-tree` output"))
        };
        let path_before = path()?;
        let path_after = if status.starts_with(['R', 'C']) {
            path()?
        } else {
            path_before.clone()
        };
        let side = |mode: &str, blob: &str, path: PathBuf| {
            mode.starts_with("100").then(|| Side {
                path,
                blob: blob.to_string(),
            })
        };
        let mut before = side(mode_before, blob_before, path_before);
        let after = side(mode_after, blob_after, path_after);
        if status.starts_with('C') {
            before = None;
        }
        changes.push(Change { before, after });
    }
    Ok(changes)
}
