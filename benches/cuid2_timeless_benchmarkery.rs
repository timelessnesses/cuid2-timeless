use criterion;
use cuid2_timeless::cuid_wrapper;
use cuid2_timeless::is_cuid;

fn one_id_bench(c: &mut criterion::Criterion) {
    c.bench_function("cuid", |b| {
        b.iter(cuid_wrapper())
    });
}

fn a_lot_id_bench(c: &mut criterion::Criterion) {
    let mut instance = cuid_wrapper();
    c.bench_function("cuid2 x 10,000", |b| {
        b.iter(|| {
            (0..10_000).for_each(|_| { // i want to test 100,000 but it's too slow
                instance().unwrap();
            })
        })
    });
}

fn checks_cuid_bench(c: &mut criterion::Criterion) {
    let mut instance = cuid_wrapper();
    c.bench_function("checkery cuid2", |b| {
        b.iter(|| {
            is_cuid(instance().unwrap(), None, None);
        })
    });
}

criterion::criterion_group!(
    cuid2_timeless,
    one_id_bench,
    a_lot_id_bench,
    checks_cuid_bench
);

criterion::criterion_main!(
    cuid2_timeless
);
