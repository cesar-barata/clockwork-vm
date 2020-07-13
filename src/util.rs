pub fn pair_result<T1, T2, E>(
    res1: std::result::Result<T1, E>,
    res2: std::result::Result<T2, E>
) -> std::result::Result<(T1, T2), E> {
    res1.and_then(|v1| res2.and_then(|v2| Ok((v1, v2))))
}