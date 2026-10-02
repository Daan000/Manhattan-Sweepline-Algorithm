use manhattan_sweepline::*;
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

    let res = sweep_line.run();
    println!("{:?}", res);
}
