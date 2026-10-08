use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::Duration;
use rand::Rng;
use manhattan_sweepline::{Line, Point, SweepLine};

// Lijnen met volledig willekeurige punten
fn generate_random_lines(n: usize) -> Vec<Line> {
    let mut rng = rand::thread_rng();
    (0..n).map(|id| {
        let x1: f64 = rng.gen_range(0.0..1000.0);
        let x2: f64 = rng.gen_range(0.0..1000.0);
        let y: f64 = rng.gen_range(0.0..1000.0);

        Line {
            id,
            start: Point { x: x1.min(x2), y },
            end: Point { x: x1.max(x2), y },
        }
    }).collect()
}

// Weinig overlap: korte lijnen verspreid over een groot gebied
fn generate_sparse_lines(n: usize) -> Vec<Line> {
    let mut rng = rand::thread_rng();
    (0..n).map(|id| {
        let x1: f64 = rng.gen_range(0.0..100_000.0);
        let length: f64 = rng.gen_range(1.0..10.0); // Zeer korte lijnen
        let x2: f64 = x1 + length;
        let y: f64 = rng.gen_range(0.0..100_000.0);

        Line {
            id,
            start: Point { x: x1, y },
            end: Point { x: x2, y },
        }
    }).collect()
}

// Veel overlap: lange lijnen in een klein gebied
fn generate_dense_lines(n: usize) -> Vec<Line> {
    let mut rng = rand::thread_rng();
    (0..n).map(|id| {
        let x1: f64 = rng.gen_range(0.0..50.0);
        let length: f64 = rng.gen_range(800.0..1000.0); // Zeer lange lijnen
        let x2: f64 = x1 + length;
        let y: f64 = rng.gen_range(0.0..10.0); // Zeer kleine Y-ruimte

        Line {
            id,
            start: Point { x: x1, y },
            end: Point { x: x2, y },
        }
    }).collect()
}

fn bench_sweepline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sweepline Time Complexity");
    let sizes = [100, 200, 400, 800, 1600, 3200, 6400, 12800];
    group.measurement_time(Duration::from_secs(20));
    for size in sizes.iter() {
        // 1. Willekeurige distributie
        group.bench_with_input(BenchmarkId::new("Random", size), size, |b, &size| {
            let lines = generate_random_lines(size);
            b.iter(|| {
                let mut sl = SweepLine::default();
                for line in lines.iter().cloned() {
                    sl.add_line(line);
                }
                black_box(sl.run())
            });
        });

        // 2. Weinig overlap distributie
        group.bench_with_input(BenchmarkId::new("Weinig Overlap", size), size, |b, &size| {
            let lines = generate_sparse_lines(size);
            b.iter(|| {
                let mut sl = SweepLine::default();
                for line in lines.iter().cloned() {
                    sl.add_line(line);
                }
                black_box(sl.run())
            });
        });

        // 3. Veel overlap distributie
        group.bench_with_input(BenchmarkId::new("Veel Overlap", size), size, |b, &size| {
            let lines = generate_dense_lines(size);
            b.iter(|| {
                let mut sl = SweepLine::default();
                for line in lines.iter().cloned() {
                    sl.add_line(line);
                }
                black_box(sl.run())
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sweepline);
criterion_main!(benches);