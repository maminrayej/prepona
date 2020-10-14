pub fn from_ij(i: usize, j: usize) -> usize {
    let k = std::cmp::max(i, j);

    (k - i) + j + k*k
}