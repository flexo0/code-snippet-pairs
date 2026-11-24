mod issue_13094 {
    fn issue_13094<T>(
        mat_a: &[Vec<T>],
        mat_b: &[Vec<T>],
        add: impl Fn(T, T) -> T,
        mul: impl Fn(T, T) -> T,
    ) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        (0..m)
                            .map(|k| mul(mat_a[i][k].clone(), mat_b[k][j].clone()))
                            .reduce(&add)
                            //~^ redundant_closure
                            .expect("Matrix must be qualified!")
                    })
                    .collect()
            })
            .collect()
    }
}
