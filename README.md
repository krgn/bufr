# bufr

I was contemplating how to create a buffer manager, which I can lend a
buffer from, put some data into and send it on to another thread for
further processing. That buffer will be automatically put back into
the managers pool once reference gets dropped.

The main use-case I imagine is a streaming server which reads data
from a socket in a loop into a `BufrRef`, then send copies of that ref
(possibly wrapped in `Arc`) to worker threads wherer it gets send to
different destinations.

Only when all threads successfully sent-off the underlying buffer it
gets dropped and returned to the pool for reuse.

This little example project is just for keeping track of that pattern,
possibly for future use in some networking server code.

```rust
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
}
```
