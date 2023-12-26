# Counter-fpy

An implementation of Python's Counter module in Rust.

[https://docs.python.org/3/library/collections.html?highlight=counter#collections.Counter](https://docs.python.org/3/library/collections.html?highlight=counter#collections.Counter)

Example

```rs

use counter_fpy::Counter;

fn main() {
    let sample = ["bxffour", "bxffour", "blackprince"];

    let counter = Counter::new();
    let collection = Counter::from(counter, &sample);

    for (key, count) in collection.iter() {
        println!("Key: {key} has count: {count}");
    }
}

```
