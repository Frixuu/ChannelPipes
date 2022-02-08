# channel_pipes

## Usage

If you use ```crossbeam-channel```, add this to your ```Cargo.toml```:

```toml
[dependencies]
channel_pipes = { version = "0.2", features = ["crossbeam"] }
```

## Examples

```rust
use channel_pipes::crossbeam::{CrossbeamSender, DistinctUntilChanged};
use crossbeam_channel::unbounded;

fn main() {
    let (s, r) = unbounded::<i32>().distinct_until_changed();

    let vec = vec![1, 2, 2, 3, 3, 3, 1];
    for i in vec {
        s.send(i);
    }

    assert_eq!(Ok(1), r.try_recv());
    assert_eq!(Ok(2), r.try_recv());
    assert_eq!(Ok(3), r.try_recv());
    assert_eq!(Ok(1), r.try_recv());
    assert!(r.try_recv().is_err());
}
```
