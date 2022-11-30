# stats

stats is a Rust statistics library

The main thing is the `Stats` trait which provides all the methods.
It is implemented for all the collection-like types in the standard
library and can be implemented for any type if that type implements
`IntoIterator` and `Clone`

## Examples

It works on `Vec`tors

```rust
use stats::Stats;

fn main() {
    let my_vec = vec![1, 2, 3];
    assert_eq!(my_vec.mean(), 2);
}

```

To get the methods on your type

```rust
use stats::Stats;

#[derive(Clone)]
struct MyStruct {
    // ...
};

impl IntoIterator for MyStruct {
    // ...
}

impl Stats for MyStruct {}

// Now we can use the methods in `Stats`

fn main() {
    let my_struct = MyStruct {};
    println!("{}", my_struct.mean());
}

```
