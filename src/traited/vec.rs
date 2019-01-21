use super::bufr::{Bufr, BufrRef};
use super::{DeQueuable, Queuable};

use std::sync::{Arc, Mutex};

pub struct LockedVec(Arc<Mutex<Vec<Bufr>>>);

impl LockedVec {
    pub fn with_capacity(size: usize) -> LockedVec {
        let mut data = Vec::with_capacity(size);

        for _n in 0..size {
            data.push(Bufr::default())
        }

        LockedVec(Arc::new(Mutex::new(data)))
    }
}

impl Clone for LockedVec {
    fn clone(&self) -> LockedVec {
        LockedVec(Arc::clone(&self.0))
    }
}

impl DeQueuable<LockedVec> for LockedVec {
    fn pop(&mut self) -> BufrRef<LockedVec> {
        self.0.lock().unwrap().pop().map_or_else(
            || BufrRef::create(self.clone()),
            |bufr| BufrRef::new(bufr, self.clone()),
        )
    }
}

impl Queuable for LockedVec {
    fn push(&mut self, buf: Bufr) {
        self.0.lock().unwrap().push(buf)
    }
}

impl Default for LockedVec {
    fn default() -> LockedVec {
        LockedVec::with_capacity(1024)
    }
}
