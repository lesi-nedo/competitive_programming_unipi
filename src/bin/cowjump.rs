use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, Read, Write};
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    a: Point,
    b: Point,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Begin,
    End,
}

#[derive(Copy, Clone, Debug)]
struct Event {
    x: i64,
    y: i64,
    kind: EventType,
    id: usize,
}

#[derive(Copy, Clone)]
struct ActiveSegment<'a> {
    id: usize,
    segments: &'a [Segment],
    sweep_x: &'a Cell<i64>,
}

impl<'a> PartialEq for ActiveSegment<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for ActiveSegment<'a> {}

impl<'a> PartialOrd for ActiveSegment<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ActiveSegment<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        }

        cmp_segments_at(self.segments, self.id, other.id, self.sweep_x.get())
    }
}

fn y_key(seg: &Segment, x: i64) -> (i128, i128) {
    let dx = (seg.b.x - seg.a.x) as i128;
    if dx == 0 {
        return (seg.a.y as i128, 1);
    }

    let dy = (seg.b.y - seg.a.y) as i128;
    let offset = (x - seg.a.x) as i128;
    let num = seg.a.y as i128 * dx + dy * offset;
    (num, dx)
}

fn cmp_segments_at(segments: &[Segment], id1: usize, id2: usize, x: i64) -> Ordering {
    let (num1, den1) = y_key(&segments[id1], x);
    let (num2, den2) = y_key(&segments[id2], x);

    (num1 * den2)
        .cmp(&(num2 * den1))
        .then_with(|| id1.cmp(&id2))
}

fn orient(a: &Point, b: &Point, c: &Point) -> i128 {
    (b.x - a.x) as i128 * (c.y - a.y) as i128 - (b.y - a.y) as i128 * (c.x - a.x) as i128
}

fn on_segment(a: &Point, b: &Point, p: &Point) -> bool {
    a.x.min(b.x) <= p.x && p.x <= a.x.max(b.x) && a.y.min(b.y) <= p.y && p.y <= a.y.max(b.y)
}

fn intersect(s1: &Segment, s2: &Segment) -> bool {
    let a = &s1.a;
    let b = &s1.b;
    let c = &s2.a;
    let d = &s2.b;

    let o1 = orient(a, b, c);
    let o2 = orient(a, b, d);
    let o3 = orient(c, d, a);
    let o4 = orient(c, d, b);

    if ((o1 > 0 && o2 < 0) || (o1 < 0 && o2 > 0)) && ((o3 > 0 && o4 < 0) || (o3 < 0 && o4 > 0)) {
        return true;
    }

    if o1 == 0 && on_segment(a, b, c) {
        return true;
    }
    if o2 == 0 && on_segment(a, b, d) {
        return true;
    }
    if o3 == 0 && on_segment(c, d, a) {
        return true;
    }
    if o4 == 0 && on_segment(c, d, b) {
        return true;
    }
    false
}

fn choose_answer(segments: &[Segment], id1: usize, id2: usize) -> usize {
    let mut count1 = 0;
    let mut count2 = 0;

    for i in 0..segments.len() {
        if i != id1 && intersect(&segments[id1], &segments[i]) {
            count1 += 1;
        }
        if i != id2 && intersect(&segments[id2], &segments[i]) {
            count2 += 1;
        }
    }

    if count1 > 1 {
        id1 + 1
    } else if count2 > 1 {
        id2 + 1
    } else {
        id1.min(id2) + 1
    }
}

fn solve(input: &str) -> String {
    let mut tokens = input.split_whitespace();
    let mut answers = Vec::new();

    while let Some(n_token) = tokens.next() {
        let n: usize = n_token.parse().unwrap();
        let mut segments = Vec::with_capacity(n);
        let mut events = Vec::with_capacity(2 * n);

        for id in 0..n {
            let mut a = Point {
                x: tokens.next().unwrap().parse().unwrap(),
                y: tokens.next().unwrap().parse().unwrap(),
            };
            let mut b = Point {
                x: tokens.next().unwrap().parse().unwrap(),
                y: tokens.next().unwrap().parse().unwrap(),
            };

            if a.x > b.x {
                std::mem::swap(&mut a, &mut b);
            }

            segments.push(Segment { a, b });
            events.push(Event {
                x: a.x,
                y: a.y,
                kind: EventType::Begin,
                id,
            });
            events.push(Event {
                x: b.x,
                y: b.y,
                kind: EventType::End,
                id,
            });
        }

        events.sort_unstable_by(|lhs, rhs| {
            lhs.x
                .cmp(&rhs.x)
                .then_with(|| lhs.kind.cmp(&rhs.kind))
                .then_with(|| lhs.y.cmp(&rhs.y))
                .then_with(|| lhs.id.cmp(&rhs.id))
        });

        let sweep_x = Cell::new(0_i64);
        let mut active: BTreeSet<ActiveSegment> = BTreeSet::new();
        let mut answer = None;

        for event in events {
            sweep_x.set(event.x);
            let key = ActiveSegment {
                id: event.id,
                segments: &segments,
                sweep_x: &sweep_x,
            };

            match event.kind {
                EventType::Begin => {
                    active.insert(key);

                    let above = active.range((Excluded(key), Unbounded)).next().copied();
                    let below = active.range(..key).next_back().copied();

                    if let Some(above) = above
                        && intersect(&segments[event.id], &segments[above.id])
                    {
                        answer = Some(choose_answer(&segments, event.id, above.id));
                        break;
                    }

                    if let Some(below) = below
                        && intersect(&segments[event.id], &segments[below.id])
                    {
                        answer = Some(choose_answer(&segments, event.id, below.id));
                        break;
                    }
                }
                EventType::End => {
                    let above = active.range((Excluded(key), Unbounded)).next().copied();
                    let below = active.range(..key).next_back().copied();

                    if let (Some(above), Some(below)) = (above, below)
                        && intersect(&segments[above.id], &segments[below.id])
                    {
                        answer = Some(choose_answer(&segments, above.id, below.id));
                        break;
                    }

                    active.remove(&key);
                }
            }
        }

        if let Some(answer) = answer {
            answers.push(answer.to_string());
        }
    }

    answers.join("\n")
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    File::open("cowjump.in")?.read_to_string(&mut input)?;

    let output = solve(&input);
    let mut file_output = File::create("cowjump.out")?;
    writeln!(file_output, "{output}")?;
    Ok(())
}
