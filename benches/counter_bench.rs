use counter_fpy::Counter;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn counter_eff_benchmark(c: &mut Criterion) {
  let sample = black_box(["bxffour", "bxffour", "blackprince"]);

  c.bench_function("testing counter objects", |_| {
    let counter = Counter::new();
    let collection = Counter::from(counter, &sample);
  });

  let counter = Counter::new();
  let collection = Counter::from(counter, &sample);

  c.bench_function("testing iterable counter", |f| {
    for (key, count) in collection.iter() {
      println!("Key: {key} has count: {count}");
    }
  });
}

criterion_group!(benches, counter_eff_benchmark);
criterion_main!(benches);
