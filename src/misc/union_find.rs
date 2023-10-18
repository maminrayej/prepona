pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // Path compression
        while self.parent[i] != i {
            let temp = self.parent[i];
            self.parent[i] = root;
            i = temp;
        }
        root
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        // Union by Rank
        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_x] = root_y;
            if self.rank[root_x] == self.rank[root_y] {
                self.rank[root_y] += 1;
            }
        }
    }

    pub fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
