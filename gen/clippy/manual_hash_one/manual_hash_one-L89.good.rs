fn msrv_1_71(b: impl BuildHasher, v: impl Hash) {
    let _ = b.hash_one(&v);
}
