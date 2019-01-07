#![allow(dead_code)]
#![feature(result_map_or_else)]

use std::sync::{Arc, Mutex};

type Buffers = Arc<Mutex<Vec<Bufr>>>;

struct Bufr {
    data: [u8; 1024],
}

impl Bufr {
    fn new() -> Bufr {
        let data = [0; 1024];
        Bufr { data }
    }
}

struct BufrRef {
    bufr: Option<Bufr>,
    mgr: Buffers,
}

impl BufrRef {
    fn new(bufr: Bufr, mgr: Buffers) -> BufrRef {
        BufrRef {
            bufr: Some(bufr),
            mgr,
        }
    }
}

impl Drop for BufrRef {
    fn drop(&mut self) {
        if let Some(buffer) = std::mem::replace(&mut self.bufr, None) {
            restore(buffer, &mut self.mgr)
        }
    }
}

struct Mgr {
    buffers: Buffers,
}

impl Mgr {
    pub fn new() -> Mgr {
        Mgr {
            buffers: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn init(mut self, count: usize) -> Mgr {
        for _ in 0..count {
            self.restore(Bufr::new())
        }
        self
    }

    pub fn count(&self) -> usize {
        self.buffers
            .lock()
            .map_or_else(|_| 0, |buffers| buffers.len())
    }

    pub fn lend(&mut self) -> BufrRef {
        self.buffers.lock().map_or_else(
            |_error| BufrRef::new(Bufr::new(), self.buffers.clone()),
            |mut buffers| {
                let bufr = buffers.pop().unwrap_or_else(Bufr::new);
                BufrRef::new(bufr, self.buffers.clone())
            },
        )
    }

    pub fn restore(&mut self, bufr: Bufr) {
        restore(bufr, &mut self.buffers)
    }
}

fn restore(bufr: Bufr, buffers: &mut Buffers) {
    buffers
        .lock()
        .map_or_else(|_| (), |mut buffers| buffers.push(bufr));
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_drop() {
        let count = 1024;
        let mut mgr = Mgr::new().init(count);
        assert_eq!(mgr.count(), count);
        let buf = mgr.lend();
        assert_eq!(mgr.count(), count - 1);
        drop(buf);
        assert_eq!(mgr.count(), count);
    }

    #[test]
    fn test_multiple_copies() {
        let count = 1024;
        let mut mgr = Mgr::new().init(count);
        assert_eq!(mgr.count(), count);
        let buf = Arc::new(mgr.lend());
        assert_eq!(mgr.count(), count - 1);
        let mut copies = vec![];
        for _ in 0..128 {
            copies.push(buf.clone())
        }
        drop(buf);
        assert_eq!(mgr.count(), count - 1);
        drop(copies);
        assert_eq!(mgr.count(), count);
    }
}
