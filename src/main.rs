extern crate collections;
extern crate test;

use std::collections::{RingBuf, TreeSet};
use std::io;
use std::os;
use std::rc::Rc;

struct History {
    q: RingBuf<Rc<String>>,
    max_size: uint,
}

impl History {
    fn new(max_size: uint) -> History {
        History {
            q: RingBuf::with_capacity(max_size),
            max_size: max_size,
        }
    }

    fn push(&mut self, s: Rc<String>, map: &mut TreeSet<Rc<String>>) {
        self.q.push_back(s);
        if self.q.len() > self.max_size {
            match self.q.pop_front() {
                None => {},
                Some(e) => { remove(map, &e); },
            }
        }
    }
}

#[inline(never)]
fn insert(map: &mut TreeSet<Rc<String>>, e: Rc<String>) {
    map.insert(e);
}

#[inline(never)]
fn remove(map: &mut TreeSet<Rc<String>>, e: &Rc<String>) {
    map.remove(e);
}

#[inline(never)]
fn contains(map: &TreeSet<Rc<String>>, e: &Rc<String>) -> bool {
    map.contains(e)
}

const HISTORY_SIZE: uint = 4096;
const CONTAINS_LOOPS: uint = 2;

fn main() {
    let mut f = io::BufferedReader::new(io::File::open(&Path::new(os::args()[1].clone())).unwrap());

    let mut map = TreeSet::new();
    // TODO: all of these will be in cache.
    let mut hist = History::new(HISTORY_SIZE);

    let mut x = 0i;

    for line in f.lines() {
        for word in line.unwrap().words() {
            let w = Rc::new(word.to_string());
            insert(&mut map, w.clone());
            hist.push(w, &mut map);
            for s in hist.q.iter().take(CONTAINS_LOOPS) {
                if contains(&mut map, s) {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
        }
    }

    println!("x = {}", x);
}
