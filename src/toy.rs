// A Toy implementation of Uncomputing.
// It save memory, but assume objects are atomic -
// that is, an object is not made up of smaller object.
// It also lack all optimization, except union find,
// Which is crucial for performance, and a bit complex.
// Basically, this is designed for Deep Learning:
// The fundamental unit is atomic tensor.

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
