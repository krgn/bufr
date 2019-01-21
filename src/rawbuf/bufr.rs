#![allow(dead_code)]

use std::mem;

#[derive(Debug)]
pub struct RawBuf {
    capacity: usize,
    data: *mut u8,
}

impl RawBuf {
    pub fn with_capacity(capacity: usize) -> RawBuf {
        let data = {
            let mut vec = Vec::with_capacity(capacity);
            for _n in 0..capacity {
                vec.push(0)
            }
            let ptr = vec.as_mut_ptr();
            mem::forget(vec);
            ptr as *mut u8
        };
        RawBuf { capacity, data }
    }

    pub fn get_vec(&self) -> Vec<u8> {
        unsafe { Vec::from_raw_parts(self.data, self.capacity, self.capacity) }
    }
}

impl Drop for RawBuf {
    fn drop(&mut self) {
        let vec = self.get_vec();
        mem::drop(vec)
    }
}

#[cfg(test)]
mod tests {

    use crate::rawbuf::*;

    #[test]
    fn mgr_test() {
        let buf = RawBuf::with_capacity(1024);
        println!("buf: {:?}", buf);
        println!("size: {:?}", std::mem::size_of::<RawBuf>())
    }
}
