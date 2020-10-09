// A Toy implementation of Uncomputing.
// It save memory, but assume objects are atomic -
// that is, an object is not made up of smaller object.
// It also lack all optimization, except union find,
// Which is crucial for performance, and a bit complex.
// Basically, this is designed for Deep Learning:
// The fundamental unit is atomic tensor.

use either::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

struct RematerializerNode<T> {
    inputs: Vec<Uncompute<T>>,
    func: Box<dyn Fn(Vec<T>) -> T>,
    compute: Duration,
}

type Rematerializer<T> = Rc<RematerializerNode<T>>;

struct UncomputeNode<T> {
    value: Either<T, Rematerializer<T>>,
    memory: usize,
    last_accessed: Instant,
    remats: Vec<Rematerializer<T>>,
}

struct Timed<T> {
    value: T,
    before: Instant,
    after: Instant
}

pub struct Uncompute<T>(Rc<RefCell<UncomputeNode<T>>>);

impl<T:Clone> Uncompute<T> {
    pub fn wrap(t: T) -> Uncompute<T> {
        Uncompute(Rc::new(RefCell::new(UncomputeNode {
            value: Left(t),
            remats: Vec::new(),
            last_accessed: Instant::now(),
            memory: 0,
        })))
    }

    pub fn unwrap(&self) -> T {
        let x = self;
        panic!("unimplemented")
    }

    pub fn compute(inputs: Vec<Uncompute<T>>, func: Box<dyn Fn(Vec<T>) -> T>) -> Uncompute<T> {
        let t = Self::remat(&inputs, &func);
        let remat = Rc::new(RematerializerNode { inputs, func, compute: t.after - t.before});
        Uncompute(Rc::new(RefCell::new(UncomputeNode {
            value: Left(t.value),
            remats: vec!(remat),
            last_accessed: t.after,
            memory: 0
        })))
    }

    fn remat(inputs: &Vec<Uncompute<T>>, func: &Box<dyn Fn(Vec<T>) -> T>) -> Timed<T> {
        let mut raw_inputs = Vec::new();
        for x in inputs {
            raw_inputs.push(x.unwrap());
        }
        let before = Instant::now();
        let value = (func)(raw_inputs);
        let after = Instant::now();
        Timed { value, before, after }
    }

    pub fn evictable(&self) -> bool {
        (*(*self.0).borrow()).value.is_left()
    }

    pub fn evict(&self) {
        assert!(self.evictable());
        panic!("unimplemented")
    }

    pub fn try_evict(&self) {
        if self.evictable() {
            self.evict()
        }
    }
}
