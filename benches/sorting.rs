use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro128Plus;
use sorting_bench::*;

const BENCHMARK_LEN: usize = 4096;

unsafe fn benchmark_sorter(
    sorter: unsafe extern "C" fn(*mut PriceLevel, usize),
    buf: &mut Vec<PriceLevel>,
    rng: &mut impl Rng,
) {
    buf.clear();
    buf.extend((0..BENCHMARK_LEN).map(|_| PriceLevel {
        price: rng.gen(),
        quantity: rng.gen(),
        exchange_id: rng.gen(),
        order_id: rng.gen(),
    }));
    unsafe { sorter(buf.as_mut_ptr(), BENCHMARK_LEN) }
    for i in 1..BENCHMARK_LEN {
        assert!((buf[i - 1].price, buf[i - 1].quantity) <= (buf[i].price, buf[i].quantity))
    }
}

unsafe fn benchmark_clamped_sorter(
    sorter: unsafe extern "C" fn(*mut PriceLevel, usize),
    buf: &mut Vec<PriceLevel>,
    rng: &mut impl Rng,
) {
    buf.clear();
    buf.extend((0..BENCHMARK_LEN).map(|_| PriceLevel {
        price: rng.gen::<u32>() % 100,
        quantity: rng.gen::<u32>() % 100,
        exchange_id: rng.gen(),
        order_id: rng.gen(),
    }));
    unsafe { sorter(buf.as_mut_ptr(), BENCHMARK_LEN) }
    for i in 1..BENCHMARK_LEN {
        assert!((buf[i - 1].price, buf[i - 1].quantity) <= (buf[i].price, buf[i].quantity))
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buf = Vec::with_capacity(BENCHMARK_LEN);
    let mut rng = Xoshiro128Plus::from_seed([2; 16]);
    c.bench_function("sort c", |b| {
        b.iter(|| unsafe { benchmark_sorter(sort_price_levels_c, &mut buf, &mut rng) })
    });
    c.bench_function("sort cpp", |b| {
        b.iter(|| unsafe { benchmark_sorter(sort_price_levels_cpp, &mut buf, &mut rng) })
    });
    c.bench_function("sort c cpp", |b| {
        b.iter(|| unsafe { benchmark_sorter(sort_price_levels_c_cpp, &mut buf, &mut rng) })
    });
    c.bench_function("sort rust", |b| {
        b.iter(|| unsafe { benchmark_sorter(sort_price_levels_rust, &mut buf, &mut rng) })
    });
    c.bench_function("sort clamped c", |b| {
        b.iter(|| unsafe { benchmark_clamped_sorter(sort_price_levels_c, &mut buf, &mut rng) })
    });
    c.bench_function("sort clamped cpp", |b| {
        b.iter(|| unsafe { benchmark_clamped_sorter(sort_price_levels_cpp, &mut buf, &mut rng) })
    });
    c.bench_function("sort clamped c cpp", |b| {
        b.iter(|| unsafe { benchmark_clamped_sorter(sort_price_levels_c_cpp, &mut buf, &mut rng) })
    });
    c.bench_function("sort clamped rust", |b| {
        b.iter(|| unsafe { benchmark_clamped_sorter(sort_price_levels_rust, &mut buf, &mut rng) })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
