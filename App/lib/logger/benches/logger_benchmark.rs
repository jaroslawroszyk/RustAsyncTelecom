use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::trace;

fn bench_logging_disabled(c: &mut Criterion) {
    log::set_max_level(log::LevelFilter::Info);

    c.bench_function("disabled_trace_macro", |b| {
        b.iter(|| {
            trace!("Expensive format: {} {:?}", black_box(123), black_box(vec![1,2,3]));
        })
    });
}

criterion_group!(benches, bench_logging_disabled);
criterion_main!(benches);