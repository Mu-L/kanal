use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[cfg(not(miri))]
#[allow(dead_code)]
pub const MESSAGES: usize = 100000;
#[cfg(miri)]
#[allow(dead_code)]
pub const MESSAGES: usize = 16;
#[allow(dead_code)]
pub const THREADS: usize = 8;

#[derive(PartialEq, Eq, Debug)]
pub struct Padded {
    pub a: bool,
    pub b: u8,
    pub c: u32,
}

#[derive(PartialEq, Eq, Debug)]
#[repr(C)]
pub struct PaddedReprC {
    pub a: bool,
    pub b: u8,
    pub c: u32,
}

pub struct DropTester {
    i: usize,
    dropped: bool,
    counter: Arc<AtomicUsize>,
}

impl Drop for DropTester {
    fn drop(&mut self) {
        if self.dropped {
            panic!("double dropped");
        }
        if self.i == 0 {
            panic!("bug: i=0 is invalid value for drop tester");
        }
        self.dropped = true;
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

impl DropTester {
    #[allow(dead_code)]
    pub fn new(counter: Arc<AtomicUsize>, i: usize) -> Self {
        if i == 0 {
            panic!("don't initialize DropTester with 0");
        }
        Self {
            i,
            dropped: false,
            counter,
        }
    }
}
