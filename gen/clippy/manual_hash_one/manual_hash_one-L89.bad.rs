fn msrv_1_71(b: impl BuildHasher, v: impl Hash) {
    let mut hasher = b.build_hasher();
    v.hash(&mut hasher);
    let _ = hasher.finish();
}
