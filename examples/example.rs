use counter_fpy::Counter;

fn main() {
    let sample = ["bxffour", "bxffour", "blackprince"];

    let counter = Counter::new();
    let collection = Counter::from(counter, &sample);

    for (key, count) in collection.iter() {
        println!("Key: {key} has count: {count}");
    }
}
