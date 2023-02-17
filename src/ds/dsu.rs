/// Disjoint sets union.
#[derive(Debug, Default, Clone)]
pub struct DSU {
    root_or_size: Vec<i32>,
}

/// Disjoint sets union index.
pub type DSUIdx = usize;

impl DSU {
    /// Create an empty DSU.
    ///
    /// # Examples
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::new();
    /// ```
    pub fn new() -> Self {
        DSU::with_capacity(0)
    }

    /// Create a DSU with `n` elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// ```
    pub fn with_capacity(n: usize) -> Self {
        Self {
            root_or_size: vec![-1; n],
        }
    }

    /// Return the representative of the group that contains the element.
    ///
    /// Return `None` if out of bounds.
    ///
    /// **Complexity** `O(log(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.root(0), dsu.root(3));
    /// ```
    pub fn root(&self, a: DSUIdx) -> Option<DSUIdx> {
        if let Some(r) = self.root_or_size.get(a) {
            if *r < 0 {
                Some(a)
            } else {
                self.root(*r as DSUIdx)
            }
        } else {
            None
        }
    }

    /// Return the representative of the group that contains the element.
    ///
    /// Return `None` if out of bounds.
    ///
    /// **Complexity** `O(a(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.root(0), dsu.root(3));
    /// ```
    pub fn root_mut(&mut self, a: DSUIdx) -> Option<DSUIdx> {
        if let Some(r) = self.root_or_size.get(a).cloned() {
            if r < 0 {
                Some(a)
            } else {
                self.root_or_size[a] = self.root_mut(r as DSUIdx).unwrap() as i32;
                Some(self.root_or_size[a] as DSUIdx)
            }
        } else {
            None
        }
    }

    /// Merge `a`'s group and `b`'s group.
    /// If `a` and `b` are in the same group, it return the representative element.
    /// Otherwise, it return the representative element of the new group.
    ///
    /// Return `None` if `a` or `b` are out of bounds.
    ///
    /// **Complexity** `O(a(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.root(0), dsu.root(3));
    /// ```
    pub fn merge(&mut self, a: DSUIdx, b: DSUIdx) -> Option<DSUIdx> {
        match (self.root(a), self.root(b)) {
            (Some(ra), Some(rb)) => {
                if ra == rb {
                    None
                } else {
                    let (ra, rb) = if self.root_or_size[ra] < self.root_or_size[rb] {
                        (ra, rb)
                    } else {
                        (rb, ra)
                    };
                    self.root_or_size[ra] += self.root_or_size[rb];
                    self.root_or_size[rb] = ra as i32;
                    Some(ra)
                }
            }
            _ => None,
        }
    }

    /// Return whether `a` and `b` are in the same group.
    ///
    /// Return `None` if `a` or `b` are out of bounds.
    ///
    /// **Complexity** `O(log(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.same(0, 3), Some(true));
    /// assert_eq!(dsu.same(0, 1), Some(false));
    /// ```
    pub fn same(&self, a: DSUIdx, b: DSUIdx) -> Option<bool> {
        match (self.root(a), self.root(b)) {
            (Some(ra), Some(rb)) => Some(ra == rb),
            _ => None,
        }
    }

    /// Return whether `a` and `b` are in the same group.
    ///
    /// Return `None` if `a` or `b` are out of bounds.
    ///
    /// **Complexity** `O(a(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.same_mut(0, 3), Some(true));
    /// assert_eq!(dsu.same_mut(0, 1), Some(false));
    /// ```
    pub fn same_mut(&mut self, a: DSUIdx, b: DSUIdx) -> Option<bool> {
        match (self.root_mut(a), self.root_mut(b)) {
            (Some(ra), Some(rb)) => Some(ra == rb),
            _ => None,
        }
    }

    /// Return the size of the group that contains `a`.
    ///
    /// Return `None` if out of bounds.
    ///
    /// **Complexity** `O(log(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.size(0), Some(2));
    /// assert_eq!(dsu.size(1), Some(1));
    /// ```
    pub fn size(&self, a: DSUIdx) -> Option<usize> {
        if let Some(r) = self.root(a) {
            Some(-self.root_or_size[r] as usize)
        } else {
            None
        }
    }

    /// Return the size of the group that contains `a`.
    ///
    /// Return `None` if out of bounds.
    ///
    /// **Complexity** `O(a(n))`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.size_mut(0), Some(2));
    /// assert_eq!(dsu.size_mut(1), Some(1));
    /// ```
    pub fn size_mut(&mut self, a: DSUIdx) -> Option<usize> {
        if let Some(r) = self.root_mut(a) {
            Some(-self.root_or_size[r] as usize)
        } else {
            None
        }
    }

    /// Return groups list.
    ///
    /// **Complexity** `O(n)`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.groups(), vec![vec![1], vec![2], vec![0, 3]]);
    /// ```
    pub fn groups(&self) -> Vec<Vec<DSUIdx>> {
        let n = self.root_or_size.len();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            g[self.root(i).unwrap()].push(i);
        }
        g.iter().cloned().filter(|x| !x.is_empty()).collect()
    }

    /// Return groups list.
    ///
    /// **Complexity** `O(n)`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::DSU;
    /// let mut dsu = DSU::with_capacity(4);
    /// dsu.merge(0, 3);
    /// assert_eq!(dsu.groups_mut(), vec![vec![1], vec![2], vec![0, 3]]);
    /// ```
    pub fn groups_mut(&mut self) -> Vec<Vec<DSUIdx>> {
        let n = self.root_or_size.len();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            g[self.root_mut(i).unwrap()].push(i);
        }
        g.iter().cloned().filter(|x| !x.is_empty()).collect()
    }
}
