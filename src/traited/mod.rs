#![allow(dead_code)]

mod bufr;
mod mgr;
mod queue;
mod vec;

pub use self::bufr::*;
pub use self::mgr::*;
pub use self::queue::*;
pub use self::vec::*;

/// # Queuable
///
/// We create this little abstraction that'll allow us to swap out
/// buffer storages.
pub trait Queuable {
    fn push(&mut self, t: bufr::Bufr);
}

/// # DeQueuable
///
/// Trait to describe a type which holds buffers to take from.
pub trait DeQueuable<T>
where
    T: Queuable,
{
    fn pop(&mut self) -> bufr::BufrRef<T>;
}
