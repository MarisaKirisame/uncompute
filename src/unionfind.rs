// A UnionFind datastructure.
// Note the additional V parameter -
// It allow a summary of the disjoint set.
// In case you dont need any summary,
// Just use V = ().
// The design and implementation is inspired by egg and ena.
use std::ops::Add;

pub struct UnionFind<V> {
    vec: Vec<UnionFindNode<V>>,
}

impl<V: Copy + Add<Output = V>> UnionFind<V> {
    pub fn merge(&mut self, mut l: UnionFindIdx, mut r: UnionFindIdx) {
        l = self.find(l);
        r = self.find(r);
        if l != r {
            self.set_parent(l, r);
            self.get_mut(r).v = self.get(l).v + self.get(r).v;
        }
    }
    pub fn idx_eq(&mut self, l: UnionFindIdx, r: UnionFindIdx) -> bool {
        self.find(l) == self.find(r)
    }
    pub fn new() -> UnionFind<V> {
        UnionFind { vec: Vec::new() }
    }
    pub fn new_class(&mut self, v: V) -> UnionFindIdx {
        let idx = UnionFindIdx {
            idx: self.vec.len(),
        };
        self.vec.push(UnionFindNode { parent: idx, v });
        idx
    }
    pub fn v(&self, idx: UnionFindIdx) -> V {
        self.get(idx).v
    }
    pub fn set_v(&mut self, idx: UnionFindIdx, v: V) {
        self.get_mut(idx).v = v;
    }
    fn get(&self, idx: UnionFindIdx) -> &UnionFindNode<V> {
        &self.vec[idx.idx]
    }
    fn get_mut(&mut self, idx: UnionFindIdx) -> &mut UnionFindNode<V> {
        &mut self.vec[idx.idx]
    }
    fn parent(&self, idx: UnionFindIdx) -> UnionFindIdx {
        self.get(idx).parent
    }
    fn set_parent(&mut self, idx: UnionFindIdx, parent: UnionFindIdx) {
        self.get_mut(idx).parent = parent
    }
    fn find(&mut self, mut idx: UnionFindIdx) -> UnionFindIdx {
        let parent = self.parent(idx);
        while idx != parent {
            let grand_parent = self.parent(parent);
            self.set_parent(idx, grand_parent);
            idx = grand_parent;
        }
        idx
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnionFindIdx {
    idx: usize,
}

struct UnionFindNode<V> {
    // If the node is root, parent is itself.
    parent: UnionFindIdx,
    v: V,
}

impl<V> UnionFindNode<V> {
    fn get(&self) -> &V {
        &self.v
    }
    fn set(&mut self, v: V) {
        self.v = v
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct NoSummary;

impl Add for NoSummary {
    type Output = NoSummary;
    fn add(self: NoSummary, other: NoSummary) -> NoSummary {
        NoSummary
    }
}

#[cfg(test)]
mod tests {
    use crate::unionfind::*;
    #[test]
    fn test() {
        let mut uf = UnionFind::<NoSummary>::new();
        let x = uf.new_class(NoSummary);
        assert!(uf.idx_eq(x, x));
        let y = uf.new_class(NoSummary);
        assert!(!uf.idx_eq(x, y));
        uf.merge(x, y);
        assert!(uf.idx_eq(x, y));
    }
}
