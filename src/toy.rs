// A Toy implementation of Uncomputing.
// It save memory, but assume objects are atomic -
// that is, an object is not made up of smaller object.
// It also lack all optimization, except union find,
// Which is crucial for performance, and a bit complex.
// Basically, this is designed for Deep Learning:
// The fundamental unit is atomic tensor.

use either::*;
use std::rc::Rc;
use std::time::{Duration, Instant};
/*

struct RematerializerNode<T> {
    inputs: Vec<mut Uncompute<T>>,
    func: Box<dyn Fn(Vec<T>) -> T>,
    compute: Duration,
}

type Rematerializer<T> = Rc<RematerializerNode<T>>;

pub struct UncomputeNode<T> {
    value: Either<T, Rematerializer<T>>,
    memory: usize,
    last_accessed: Instant,
    remats: Vec<Rematerializer<T>>,
}

pub struct Uncompute<T>(Rc<Cell<Uncompute<T>>>);

impl<T> Uncompute<T> {
    pub fn wrap(t: T) -> Uncompute<T> {
        Uncompute {
            value: Left(t),
            remats: Vec::new(),
            last_accessed: Instant::now(),
            memory: 0,
        }
    }
    pub fn unwrap(&mut self) -> T where T:Clone {
        match &self.value {
            Left(t) => t.clone(),
            Right(r) => {
                self.value = Right(r.clone());
                panic!("unimplemented")
            }
        }
    }
    pub fn compute(input: Vec<Uncompute<T>>, func: Box<dyn Fn(Vec<T>) -> T>) {

    }
    pub fn evictable(self: Uncompute<T>) -> bool {
        self.remat.is_none()
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
 */
