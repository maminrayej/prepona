use std::cmp;

/// `UnionFind<K>` is a disjoint-set data structure. It tracks set membership of *n* elements
/// indexed from *0* to *n - 1*. The scalar type is `K` which must be an unsigned integer type.
///
/// <http://en.wikipedia.org/wiki/Disjoint-set_data_structure>
///
/// Too awesome not to quote:
///
/// “The amortized time per operation is **O(α(n))** where **α(n)** is the
/// inverse of **f(x) = A(x, x)** with **A** being the extremely fast-growing Ackermann function.”
#[derive(Debug, Clone)]
pub struct UnionFind {
    // For element at index *i*, store the index of its parent; the representative itself
    // stores its own index. This forms equivalence classes which are the disjoint sets, each
    // with a unique representative.
    parent: Vec<usize>,
    // It is a balancing tree structure,
    // so the ranks are logarithmic in the size of the container -- a byte is more than enough.
    //
    // Rank is separated out both to save space and to save cache in when searching in the parent
    // vector.
    rank: Vec<u8>,
}

#[inline]
unsafe fn get_unchecked(xs: &[usize], index: usize) -> &usize {
    debug_assert!(index < xs.len());
    xs.get_unchecked(index)
}

#[inline]
unsafe fn get_unchecked_mut(xs: &mut [usize], index: usize) -> &mut usize {
    debug_assert!(index < xs.len());
    xs.get_unchecked_mut(index)
}

impl UnionFind {
    /// Create a new `UnionFind` of `n` disjoint sets.
    pub fn new(n: usize) -> Self {
        let rank = vec![0; n];
        let parent = (0..n).collect();

        UnionFind { parent, rank }
    }

    /// Return the representative for `x`.
    ///
    /// **Panics** if `x` is out of bounds.
    pub fn find(&self, x: usize) -> usize {
        assert!(x < self.parent.len());
        unsafe {
            let mut x = x;
            loop {
                // Use unchecked indexing because we can trust the internal set ids.
                let xparent = *get_unchecked(&self.parent, x);
                if xparent == x {
                    break;
                }
                x = xparent;
            }
            x
        }
    }

    /// Return the representative for `x`.
    ///
    /// Write back the found representative, flattening the internal
    /// datastructure in the process and quicken future lookups.
    ///
    /// **Panics** if `x` is out of bounds.
    pub fn find_mut(&mut self, x: usize) -> usize {
        assert!(x < self.parent.len());
        unsafe { self.find_mut_recursive(x) }
    }

    unsafe fn find_mut_recursive(&mut self, mut x: usize) -> usize {
        let mut parent = *get_unchecked(&self.parent, x);
        while parent != x {
            let grandparent = *get_unchecked(&self.parent, parent);
            *get_unchecked_mut(&mut self.parent, x) = grandparent;
            x = parent;
            parent = grandparent;
        }
        x
    }

    /// Returns `true` if the given elements belong to the same set, and returns
    /// `false` otherwise.
    pub fn equiv(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Unify the two sets containing `x` and `y`.
    ///
    /// Return `false` if the sets were already the same, `true` if they were unified.
    ///
    /// **Panics** if `x` or `y` is out of bounds.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        if x == y {
            return false;
        }
        let xrep = self.find_mut(x);
        let yrep = self.find_mut(y);

        if xrep == yrep {
            return false;
        }

        let xrepu = xrep;
        let yrepu = yrep;
        let xrank = self.rank[xrepu];
        let yrank = self.rank[yrepu];

        // The rank corresponds roughly to the depth of the treeset, so put the
        // smaller set below the larger
        match xrank.cmp(&yrank) {
            cmp::Ordering::Less => self.parent[xrepu] = yrep,
            cmp::Ordering::Greater => self.parent[yrepu] = xrep,
            cmp::Ordering::Equal => {
                self.parent[yrepu] = xrep;
                self.rank[xrepu] += 1;
            }
        }
        true
    }
}
