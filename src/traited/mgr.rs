use super::Queuable;

pub struct Mgr<T>
where
    T: Queuable + Default,
{
    pub buffers: T,
}

impl<T> Mgr<T>
where
    T: Queuable + Default,
{
    pub fn new() -> Mgr<T> {
        let buffers = T::default();
        Mgr { buffers }
    }
}

#[cfg(test)]
mod tests {

    use crate::traited::*;

    #[test]
    fn basic_queue_test() {
        let mut mgr: Mgr<Queue> = Mgr::new();
        let _buf_ref = mgr.buffers.pop();
    }

    #[test]
    fn basic_locked_vec_test() {
        let mut mgr: Mgr<LockedVec> = Mgr::new();
        let _buf_ref = mgr.buffers.pop();
    }
}
