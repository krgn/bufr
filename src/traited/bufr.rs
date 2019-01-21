#![allow(dead_code)]

use super::*;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

const BUF_SIZE: usize = 1024 * 1024 * 16;

//  ____         __
// | __ ) _   _ / _|_ __
// |  _ \| | | | |_| '__|
// | |_) | |_| |  _| |
// |____/ \__,_|_| |_|

pub struct Bufr(Vec<u8>);

impl Bufr {
    pub fn new() -> Bufr {
        Bufr(Vec::with_capacity(BUF_SIZE))
    }
}

impl Default for Bufr {
    fn default() -> Bufr {
        Bufr::new()
    }
}

//  ____         __ ____       __
// | __ ) _   _ / _|  _ \ ___ / _|
// |  _ \| | | | |_| |_) / _ \ |_
// | |_) | |_| |  _|  _ <  __/  _|
// |____/ \__,_|_| |_| \_\___|_|

pub struct BufrRefInner<T>
where
    T: Queuable,
{
    bufr: Option<Bufr>,
    manager: T,
}

impl<T> BufrRefInner<T>
where
    T: Queuable,
{
    pub fn new(bufr: Bufr, manager: T) -> BufrRefInner<T> {
        BufrRefInner {
            bufr: Some(bufr),
            manager,
        }
    }
}

impl<T> Drop for BufrRefInner<T>
where
    T: Queuable,
{
    fn drop(&mut self) {
        if let Some(buffer) = std::mem::replace(&mut self.bufr, None) {
            self.manager.push(buffer)
        }
    }
}

impl<T> Deref for BufrRefInner<T>
where
    T: Queuable,
{
    type Target = Bufr;
    fn deref(&self) -> &Self::Target {
        if let Some(bufr) = &self.bufr {
            return &bufr;
        }
        panic!("re-use of de-allocated BufrRefInner")
    }
}

impl<T> DerefMut for BufrRefInner<T>
where
    T: Queuable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(ref mut bufr) = &mut self.bufr {
            return &mut *bufr;
        }
        panic!("re-use of de-allocated BufrRefInner")
    }
}

//  ____         __      ____       __
// | __ ) _   _ / _|_ __|  _ \ ___ / _|
// |  _ \| | | | |_| '__| |_) / _ \ |_
// | |_) | |_| |  _| |  |  _ <  __/  _|
// |____/ \__,_|_| |_|  |_| \_\___|_|

pub struct BufrRef<T: Queuable>(Arc<BufrRefInner<T>>);

impl<T> BufrRef<T>
where
    T: Queuable,
{
    pub fn new(bufr: Bufr, manager: T) -> BufrRef<T> {
        BufrRef(Arc::new(BufrRefInner::new(bufr, manager)))
    }

    pub fn create(manager: T) -> BufrRef<T> {
        let bufr = Bufr::default();
        BufrRef::new(bufr, manager)
    }
}

impl<T> Clone for BufrRef<T>
where
    T: Queuable,
{
    fn clone(&self) -> BufrRef<T> {
        BufrRef(Arc::clone(&self.0))
    }
}
