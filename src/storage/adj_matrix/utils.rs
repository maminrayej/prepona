pub fn from_ij(mut i: usize, mut j: usize, is_directed: bool) -> usize {
    if is_directed {
        let k = std::cmp::max(i, j);

        (k - i) + j + k * k
    } else {
        if j > i {
            std::mem::swap(&mut i, &mut j);
        }
        (i * (i + 1) >> 1) + j
    }
}
