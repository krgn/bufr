use super::bufr::{Bufr, BufrRef};
use super::{DeQueuable, Queuable};

use std::sync::mpsc::{self, Receiver, Sender};

pub struct ReturnEnd(Sender<Bufr>);

impl Queuable for ReturnEnd {
    fn push(&mut self, buf: Bufr) {
        self.0.send(buf).expect("could not return buffer to queue")
    }
}

impl Clone for ReturnEnd {
    fn clone(&self) -> ReturnEnd {
        ReturnEnd(self.0.clone())
    }
}

pub struct Queue(ReturnEnd, Receiver<Bufr>);

impl Queue {
    pub fn with_capacity(size: usize) -> Queue {
        let (sender, receiver) = mpsc::channel();

        for _n in 0..size {
            sender
                .send(Bufr::new())
                .expect("unable to initialize Queue");
        }

        Queue(ReturnEnd(sender), receiver)
    }
}

impl DeQueuable<ReturnEnd> for Queue {
    fn pop(&mut self) -> BufrRef<ReturnEnd> {
        self.1.try_recv().map_or_else(
            |_err| BufrRef::create(self.0.clone()),
            |bufr| BufrRef::new(bufr, self.0.clone()),
        )
    }
}

impl Queuable for Queue {
    fn push(&mut self, buf: Bufr) {
        self.0.push(buf)
    }
}

impl Default for Queue {
    fn default() -> Queue {
        Queue::with_capacity(1024)
    }
}
