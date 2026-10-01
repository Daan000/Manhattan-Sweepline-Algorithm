use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .total_cmp(&other.x)
            .then_with(|| self.y.total_cmp(&other.y))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub id: usize,
    pub start: Point,
    pub end: Point,
}
impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .y
            .partial_cmp(&other.start.y)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Line {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventType {
    Start(Line),
    End(Line),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    point: Point,
    event_type: EventType,
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Draai other en self om zodat het linkste element eerst komt
        other.point.cmp(&self.point)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub struct SweepLine {
    event_queue: BinaryHeap<Event>,
    active_lines: BTreeSet<Line>,
    start_x: f64,
    result: f64,
}
impl SweepLine {
    fn new() -> Self {
        SweepLine {
            event_queue: BinaryHeap::new(),
            active_lines: BTreeSet::new(),
            start_x: 0.0,
            result: 0.0,
        }
    }

    pub fn add_line(&mut self, mut line: Line) {
        if line.start.y != line.end.y {
            panic!("Lines must be horizontal");
        }
        if line.start > line.end {
            std::mem::swap(&mut line.start, &mut line.end);
        }

        self.event_queue.push(Event {
            point: line.start,
            event_type: EventType::Start(line),
        });

        self.event_queue.push(Event {
            point: line.end,
            event_type: EventType::End(line),
        });
    }

    pub fn run(&mut self) -> f64 {
        while let Some(event) = self.event_queue.pop() {
            self.process_event(event);
        }
        self.result
    }

    fn process_event(&mut self, event: Event) {
        let max_height = self
            .active_lines
            .last()
            .map(|line| line.start.y)
            .unwrap_or(0.0);

        let current_x = event.point.x;
        self.result += (current_x - self.start_x) * max_height;
        self.start_x = current_x;

        match event.event_type {
            EventType::Start(line) => {
                self.active_lines.insert(line);
            }
            EventType::End(line) => {
                self.active_lines.remove(&line);
            }
        }
    }
}
impl Default for SweepLine {
    fn default() -> Self {
        Self::new()
    }
}
