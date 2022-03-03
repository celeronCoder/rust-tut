use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    // generic impletantion of DataStructure
    pub fn new(iter: Iter) -> Self {
        // constructor method
        return Progress {
            iter,
            i: 0,
            bound: Unbounded,
        };
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        // only be called with fixed bound iterator

        let bound: Bounded = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };

        return Progress {
            iter: self.iter,
            i: self.i,
            bound: bound,
        };
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, _delims: (char, char)) -> Self {
        self.bound.delims = _delims;
        return self;
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
// generically implemetnting progress as an iterator
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;

        return self.iter.next();
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
    // only implement for iterable types
{
    fn progress(self) -> Progress<Self, Unbounded> {
        return Progress::new(self);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    let brkts = ('<', '>');

    for n in Progress::new(v.iter()) {
        // similar to python tqdm
        expensive_calculation(n);
    }

    // TODO: something like this v.iter().progress()
    for n in v.iter().progress().with_bound().with_delims(brkts) {
        expensive_calculation(n);
    }

    for n in (0..).progress() {
        expensive_calculation(&n);
    }
}
