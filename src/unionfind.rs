// A UnionFind datastructure.
// Note the additional V parameter -
// It allow a summary of the disjoint set.
// In case you dont need any summary,
// Just use V = ().
// The design and implementation is inspired by egg and ena.
use std::ops::Add;

struct UnionFind<V> {
    vec: Vec<UnionFindNode<V>>,
}

impl<V: Copy + Add<Output = V>> UnionFind<V> {
    pub fn merge(&mut self, mut l: UnionFindIdx, mut r: UnionFindIdx) {
        l = self.find(l);
        r = self.find(r);
        if l.idx != r.idx {
            self.set_parent(l, r);
            self.get_mut(r).v = self.get(l).v + self.get(r).v;
        }
    }
    pub fn new(&mut self, v: V) -> UnionFindIdx {
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
        self.get_mut(idx).parent = idx
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }
}
