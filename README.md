# re-init-rc
This crate provides two wrappers for Rc and Arc weak pointers - `ReInitRc` and `ReInitArc`, both types provides an API to get a pointer (`Rc` or `Arc`) from inner weak pointer (upgrade) or re-create the value using the provided function (if inner weak pointer points to a dropped value).

The API is:
```rust
impl ReInitRc<T, F: FnMut() -> T> {
    fn new(init_fn: F) -> Self { ... }
    fn get(&mut self) -> Rc<T> { ... }
}
```
And same for `ReInitArc` but `ReInitArc::get` returns `Arc<T>`

## Example with `ReInitRc` (`ReInitArc` works the same way)
The code:
```rust
use re_init_rc::ReInitRc;

struct PrintOnDrop;

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("Printing on drop")
    }
}

fn main() {
    let mut x = ReInitRc::new(|| {
        println!("Initializing new PrintOnDrop...");
        PrintOnDrop
    });
    let x1 = x.get(); // Initializing new PrintOnDrop...
    let x2 = x.get();
    drop(x1);
    drop(x2); // Printing on drop
    let x3 = x.get(); // Initializing new PrintOnDrop...
    // As `x3` is just a `Rc<PrintOnDrop>` we can also clone it
    let x4 = x3.clone();
    drop(x3);
    drop(x4); // Print on drop
}
```

Outputs:
```
Initializing new PrintOnDrop...
Printing on drop
Initializing new PrintOnDrop...
Printing on drop
```

