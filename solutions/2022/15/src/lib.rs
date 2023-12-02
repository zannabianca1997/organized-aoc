use std::{collections::HashSet, fmt::Display, isize, iter::once};

use lazy_static::lazy_static;
use regex::Regex;

/// List of segments, non overlapping
/// Segment are [.0,.1)
struct Segments(Vec<(isize, isize)>);
impl Segments {
    fn new() -> Self {
        Self(vec![])
    }

    /// add a segment to the collection
    fn add(&mut self, mut segment: (isize, isize)) {
        assert!(segment.0 <= segment.1, "Segments should be sorted");
        if segment.0 == segment.1 {
            return; // Empty segment added
        }
        // split the segments in before, after, and on the border
        let mut segment_before = vec![];
        let mut segment_after = vec![];
        for prev_segment in self.0.drain(..) {
            // is it before?
            if prev_segment.1 < segment.0 {
                segment_before.push(prev_segment)
            } else
            // is it after?
            if prev_segment.0 > segment.1 {
                segment_after.push(prev_segment)
            } else
            // is it inside?
            if prev_segment.0 >= segment.0 && prev_segment.1 <= segment.1 {
                // drop it
            } else {
                // it touches it, but is not inside
                // let's extend the segment we are adding

                // we need to enlarge the start?
                if prev_segment.0 < segment.0 {
                    segment.0 = prev_segment.0
                }
                // then we need to enlarge the end?
                if prev_segment.1 > segment.1 {
                    segment.1 = prev_segment.1
                }
            }
        }

        // collecting everything
        self.0 = segment_before
            .into_iter()
            .chain(once(segment))
            .chain(segment_after.into_iter())
            .collect();
    }

    fn total_len(&self) -> usize {
        self.0.iter().map(|(s, e)| (e - s) as usize).sum()
    }
}
impl Display for Segments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Segments {{")?;
        for (s, e) in self.0.iter() {
            write!(f, "{s}<->{e}, ")?
        }
        write!(f, "}}")
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(
            r"(?m)^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
        )
        .unwrap();
    }
    LINE_RE
        .captures_iter(input)
        .map(|capture| -> ((isize, isize), (isize, isize)) {
            (
                (
                    capture.get(1).unwrap().as_str().parse().unwrap(),
                    capture.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    capture.get(3).unwrap().as_str().parse().unwrap(),
                    capture.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
}

fn find_radii(
    sensors: impl Iterator<Item = ((isize, isize), (isize, isize))>,
) -> impl Iterator<Item = ((isize, isize), isize)> {
    sensors.map(|(p1, p2)| (p1, manhattan(p1, p2)))
}

fn manhattan(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub fn part1(input: &str) -> usize {
    const LINE_Y: isize = 2000000;
    let mut impossible_segments = Segments::new();
    let mut beacons_on_line = HashSet::new();
    for (sensor, beacon) in parse_input(input) {
        let radius = manhattan(sensor, beacon);
        // check if the beacon is on the line
        if beacon.1 == LINE_Y {
            beacons_on_line.insert(beacon.0);
        }
        // calculate the segment remaining
        let residual_radius = radius - (sensor.1 - LINE_Y).abs();
        if residual_radius >= 0 {
            let segment = (sensor.0 - residual_radius, sensor.0 + residual_radius + 1);
            impossible_segments.add(segment);
        }
    }
    impossible_segments.total_len() - beacons_on_line.len()
}
/// Solution with beacon border intersections
/// We assume the solution is unique => is constrained by at least 2 sensors boder, or an angle.
/// Also, we do all with a single call to laxy iterators chain. Cause no one should read this code
///
/// PS: f**k you old me, i had to read this code.
pub fn part2(input: &str) -> isize {
    const SQUARE_SIZE: isize = 4000000;
    let sensors: Vec<_> = find_radii(parse_input(input)).collect();

    // find all papable points
    sensors[..sensors.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(id1, (s1, r1))| {
            sensors[id1 + 1..]
                .iter()
                .filter(move |(s2, r2)| {
                    let s_dist = manhattan(*s1, *s2);
                    s_dist < r1 + r2 && s_dist + r1 > *r2 && s_dist + r2 > *r1
                })
                .flat_map(move |(s2, r2)| {
                    // iterating over all the solutions
                    [
                        (
                            r1 - r2 - s1.0 - s1.1 + s2.0 + s2.1,
                            -r1 + r2 + s1.0 - s1.1 - s2.0 + s2.1,
                            (-1, 1, -1, -1),
                        ),
                        (
                            r1 - r2 - s1.0 + s1.1 + s2.0 - s2.1,
                            -r1 + r2 + s1.0 + s1.1 - s2.0 - s2.1,
                            (-1, -1, -1, 1),
                        ),
                        (
                            2 + r1 + r2 + s1.0 + s1.1 - s2.0 - s2.1,
                            2 + r1 + r2 + s1.0 - s1.1 - s2.0 + s2.1,
                            (1, -1, -1, -1),
                        ),
                        (
                            2 + r1 + r2 + s1.0 - s1.1 - s2.0 + s2.1,
                            2 + r1 + r2 + s1.0 + s1.1 - s2.0 - s2.1,
                            (1, 1, -1, 1),
                        ),
                        (
                            2 + r1 + r2 - s1.0 + s1.1 + s2.0 - s2.1,
                            2 + r1 + r2 - s1.0 - s1.1 + s2.0 + s2.1,
                            (-1, -1, 1, -1),
                        ),
                        (
                            2 + r1 + r2 - s1.0 - s1.1 + s2.0 + s2.1,
                            2 + r1 + r2 - s1.0 + s1.1 + s2.0 - s2.1,
                            (-1, 1, 1, 1),
                        ),
                        (
                            r1 - r2 + s1.0 - s1.1 - s2.0 + s2.1,
                            -r1 + r2 - s1.0 - s1.1 + s2.0 + s2.1,
                            (1, 1, 1, -1),
                        ),
                        (
                            r1 - r2 + s1.0 + s1.1 - s2.0 - s2.1,
                            -r1 + r2 - s1.0 + s1.1 + s2.0 - s2.1,
                            (1, -1, 1, 1),
                        ),
                    ]
                    .into_iter()
                    .filter(|(d1, d2, _)| d1 % 2 == 0 && d2 % 2 == 0)
                    .filter_map(|(d1, d2, a)| {
                        let (d1, d2) = (d1 / 2, d2 / 2);
                        debug_assert_eq!(s1.0 + a.0 * (r1 + 1 - d1), s2.0 + a.2 * (r2 + 1 - d2));
                        debug_assert_eq!(s1.1 + a.1 * d1, s2.1 + a.3 * d2);
                        if 0 <= d1 && d1 <= r1 + 1 && 0 <= d2 && d2 <= r2 + 1 {
                            Some((s1.0 + a.0 * (r1 + 1 - d1), s1.1 + a.1 * d1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                })
        })
        .filter(|pos| pos.0 >= 0 && pos.0 <= SQUARE_SIZE && pos.1 >= 0 && pos.1 <= SQUARE_SIZE)
        .chain([
            // edges are special cause they can be constrict by a single sensor
            (0, 0),
            (0, SQUARE_SIZE),
            (SQUARE_SIZE, 0),
            (SQUARE_SIZE, SQUARE_SIZE),
        ])
        // now we check that the points are not inside any other sensor (so it's a valid points)
        .filter(|pts| {
            sensors
                .iter()
                .all(|(sensor, radius)| manhattan(*sensor, *pts) > *radius)
        })
        // we finally take the first point
        .next()
        .map(|(x, y)| x * 4000000 + y)
        .unwrap()
}

/*

/// Solution with iterator over beacon borders
pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let sensors: Vec<_> = find_radii(parse_input(input)).collect();
    const SQUARE_SIZE: isize = 4000000;

    // there must be a point that's papable on the border of the iterators
    let papable_pts = sensors
        .iter()
        .flat_map(|(pos, radius)| {
            (0..=radius + 1).flat_map(move |d| {
                [
                    (pos.0 + (radius + 1 - d), pos.1 + d),
                    (pos.0 + (radius + 1 - d), pos.1 - d),
                    (pos.0 - (radius + 1 - d), pos.1 + d),
                    (pos.0 - (radius + 1 - d), pos.1 - d),
                ]
            })
        })
        .filter(|pos| pos.0 >= 0 && pos.0 <= SQUARE_SIZE && pos.1 >= 0 && pos.1 <= SQUARE_SIZE);
    // now we filter the points searching for the one thats outside the radius of everyone, not only one
    let mut ok_pts = papable_pts.filter(|pts| {
        sensors
            .iter()
            .all(|(sensor, radius)| manhattan(*sensor, *pts) > *radius)
    });
    // we then take the first point found
    if let Some(pts) = ok_pts.next() {
        Ok(PuzzleResult::Numeric((pts.0 * 4000000 + pts.1) as _))
    } else {
        Err("Did not find a free position".into())
    }
}
*/

/*
//  === Gradient descend solution ===
//  Runs about 9x slower than the border search one

/// Iterate over all the point in the square [min_x, max_x)x[min_y, max_y)
/// Jumps all over the place to ensure maximum probability of finding a good start place
struct PointsOn {
    frame_stack: VecDeque<(bool, isize, isize, isize, isize)>,
}
impl PointsOn {
    fn new(min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> Self {
        assert!(min_x < max_x && min_y < max_y);
        Self {
            frame_stack: VecDeque::from([(true, min_x, max_x, min_y, max_y)]),
        }
    }
}
impl Iterator for PointsOn {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((yield_min_corner, min_x, max_x, min_y, max_y)) =
            self.frame_stack.pop_front()
        {
            // yield_min_corner is if we have to give the (min_x, min_y) corner
            debug_assert!(min_x < max_x && min_y < max_y);
            match (max_x - min_x == 1, max_y - min_y == 1) {
                (true, true) => {
                    // only one point, no more splitting
                }
                (true, false) => {
                    // split on the y axis
                    let mid_y = (max_y + min_y) / 2;
                    self.frame_stack
                        .push_back((false, min_x, max_x, min_y, mid_y));
                    self.frame_stack
                        .push_back((true, min_x, max_x, mid_y, max_y));
                }
                (false, true) => {
                    // split on the x axis
                    let mid_x = (max_x + min_x) / 2;
                    self.frame_stack
                        .push_back((false, min_x, mid_x, min_y, max_y));
                    self.frame_stack
                        .push_back((true, mid_x, max_x, min_y, max_y));
                }
                (false, false) => {
                    // split on both axis
                    let mid_x = (max_x + min_x) / 2;
                    let mid_y = (max_y + min_y) / 2;
                    self.frame_stack
                        .push_back((false, min_x, mid_x, min_y, mid_y));
                    self.frame_stack
                        .push_back((true, mid_x, max_x, min_y, mid_y));
                    self.frame_stack
                        .push_back((true, min_x, mid_x, mid_y, max_y));
                    self.frame_stack
                        .push_back((true, mid_x, max_x, mid_y, max_y));
                }
            }
            if yield_min_corner {
                return Some((min_x, min_y));
            }
        }
        None
    }
}

/// Solution with gradient descend
pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let sensors: Vec<_> = find_radii(parse_input(input)).collect();
    // calculate a score that's 0 only on the possible places
    // it's made up of piramids with the point on the sensor, and flatting out over the beacon radius
    let score = |pos: (isize, isize)| -> usize {
        sensors
            .iter()
            .map(|(sensor, radii)| (radii - manhattan(pos, *sensor) + 1).max(0))
            .max()
            .unwrap() as usize
    };

    // running gradient descend
    const SQUARE_SIZE: isize = 4000000;
    let mut visited: HashSet<_> = HashSet::new();
    'paths: for start_pos in PointsOn::new(0, SQUARE_SIZE + 1, 0, SQUARE_SIZE + 1) {
        let mut pos = start_pos;
        let mut evaluation = score(pos);
        let start_eval = evaluation;

        while evaluation > 0 {
            visited.insert(pos); // remember the path I took, even between iterations. This avoid the repeat of the same paths over and over

            // search for a better square
            for i in -1..=1 {
                for j in -1..=1 {
                    if (i, j) != (0, 0) {
                        let new_pos = (pos.0 + i, pos.1 + j);
                        if new_pos.0 >= 0
                            && new_pos.0 <= SQUARE_SIZE
                            && new_pos.1 >= 0
                            && new_pos.1 <= SQUARE_SIZE
                            && !visited.contains(&new_pos)
                        {
                            // check if the new pos is better
                            let new_evaluation = score(new_pos);
                            if new_evaluation < evaluation {
                                pos = new_pos;
                                evaluation = new_evaluation;
                            }
                        }
                    }
                }
            }

            // check we did not get stuck
            if visited.contains(&pos) {
                // we did not find a 0, or we came back on an old path. let's try with another starting point.
                continue 'paths;
            }
        }
        dbg!(start_eval);
        //Found a point with a score of 0, external from all the others!
        return Ok(PuzzleResult::Numeric((pos.0 * 4000000 + pos.1) as _));
    }

    return Err("Did not find a free position".into());
}

*/
