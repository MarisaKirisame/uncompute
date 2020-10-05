// A Toy implementation of Uncomputing.
// It save memory, but assume objects are atomic -
// that is, an object is not made up of smaller object.
// It also lack all optimization, except union find,
// Which is crucial for performance, and a bit complex.
// Basically, this is designed for Deep Learning:
// The fundamental unit is atomic tensor.

// A UnionFind datastructure.
// Note the additional V parameter -
// It allow a summary of the disjoint set.
// In case you dont need any summary,
// Just use V = ().
// The design and implementation is inspired by egg and ena.
pub struct UnionFind<V> {
    vec: Vec<UnionFindNode<V>>
}

impl<V> UnionFind<V> {
    pub fn merge(&mut self, l: UnionFindIdx, r: UnionFindIdx) {
        
    }
    pub fn new(&mut self, v: V) -> UnionFindIdx {
        let idx = UnionFindIdx { idx: self.vec.len() };
        self.vec.push(UnionFindNode { parent: idx, v });
        idx
    }
    fn find(&mut self, idx: UnionFindIdx) -> &UnionFindNode<V> {
        &self.vec[idx.idx]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnionFindIdx {
    idx: usize
}

struct UnionFindNode<V> {
    // If the node is root, parent is itself.
    parent: UnionFindIdx,
    v: V
}

impl<V> UnionFindNode<V> {
    pub fn get(&self) -> &V {
        &self.v
    }
    pub fn set(&mut self, v: V) {
        self.v = v
    }
}

struct Closure<T> {
    inputs: Vec<Uncompute<T>>,
    func: Box<dyn Fn(Vec<T>) -> T>
}

pub struct Uncompute<T> {
    t: T,
    clos: Option<Closure<T>>
}

impl<T> Uncompute<T> {
    pub fn wrap(t: T) -> Uncompute<T> {
        Uncompute { t, clos: None }
    }
    pub fn unwrap(self: Uncompute<T>) -> T {
        self.t
    }
    pub fn compute(input: Vec<Uncompute<T>>, func: Box<dyn Fn(Vec<T>) -> T>) {
        
    }
    pub fn evictable(self: Uncompute<T>) -> bool {
        self.clos.is_none()
    }
    pub fn evict() {
        
    }
    pub fn try_evict(uc: Uncompute<T>) {
        assert!(uc.evictable())
    }
    pub fn lock() {
        
    }
    pub fn unlock() {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
