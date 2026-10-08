# Manhattan Sweep Line

A Rust implementation of a sweepline algorithm for calculating the area of a Manhattan skyline.
It computes the area under overlapping lines, where each line is defined by a start and end point with the same y-coordinate.
## Usage

Add segments to a `SweepLine`, then call `run()`:

```rust
use manhattan_sweepline::{Line, Point, SweepLine};

fn main() {
    let mut sweep_line = SweepLine::default();

    sweep_line.add_line(Line {
        id: 1,
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 5.0, y: 1.0 },
    });
    sweep_line.add_line(Line {
        id: 2,
        start: Point { x: 2.0, y: 2.0 },
        end: Point { x: 4.0, y: 2.0 },
    });

    assert_eq!(sweep_line.run(), 6.0);
}
```

`Line` segments must be horizontal. If the start point has a larger x
coordinate than the end point, the coordinates are switched.
Adding a non-horizontal line causes `add_line` to panic.

## How it works

1. Each segment is converted into a start event and an end event.
2. Events are processed from left to right using a priority queue (`BinaryHeap`).
3. A `BTreeSet` stores the currently active segments, ordered by height.
4. Before each event, the area is calculated and added to the result.

With `n` segments, the sweep processes `2n` events and uses
`O(n log n)` time and `O(n)` additional space.

## Development

Run the test suite:

```bash
cargo test
```

Run the configured Criterion benchmarks:

```bash
cargo bench
```

Benchmark reports and graphs (for the report) are generated in `target/criterion/`.

