// A Toy implementation of UnComputing.
// It save memory, but assume objects are atomic -
// that is, an object is not made up of smaller object.
// It also lack all optimization, except union find,
// Which is crucial for performance, and a bit complex.
// Basically, this is designed for Deep Learning:
// The fundamental unit is atomic tensor.

struct Closure<T> {
    inputs: Vec<UC<T>>,
    func: Box<dyn Fn(Vec<T>) -> T>
}

pub struct UC<T> {
    t: T,
    clos: Option<Closure<T>>
}

impl<T> UC<T> {
    pub fn wrap(t: T) -> UC<T> {
        UC { t, clos: None }
    }
    pub fn unwrap(self: UC<T>) -> T {
        self.t
    }
    pub fn compute(input: Vec<UC<T>>, func: Box<dyn Fn(Vec<T>) -> T>) {
        
    }
    pub fn evictable(self: UC<T>) -> bool {
        self.clos.is_none()
    }
    pub fn evict() {
        
    }
    pub fn try_evict(uc: UC<T>) {
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
