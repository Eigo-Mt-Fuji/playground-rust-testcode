// https://docs.rs/bencher/latest/bencher/

#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn a(bench: &mut Bencher) {
    bench.iter(|| {
        (0..1000).fold(0, |x, y| x + y)
    })
}

benchmark_group!(benches, a);
benchmark_main!(benches);