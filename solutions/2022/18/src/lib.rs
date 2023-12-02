use std::borrow::Borrow;

fn parse_input(input: &str) -> Vec<(isize, isize, isize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let values = line
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect::<Vec<_>>();
            if values.len() == 3 {
                (values[0], values[1], values[2])
            } else {
                panic!("Expected 3 values on a line, not {}", values.len())
            }
        })
        .collect()
}

#[inline]
fn neighbours((x, y, z): (isize, isize, isize)) -> impl Iterator<Item = (isize, isize, isize)> {
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
    .into_iter()
}

pub fn part1(input: &str) -> usize {
    let rock_cubes: Vec<(isize, isize, isize)> = parse_input(input);
    let (bb, cubes) = make_grid(&rock_cubes);

    let mut faces = 0;
    for cube in rock_cubes {
        // adding the free faces
        faces += 6 - neighbours(cube)
            .filter(|neighbour| cubes[bb.idx(*neighbour)])
            .count()
    }
    faces
}

pub fn part2(input: &str) -> usize {
    let cubes: Vec<(isize, isize, isize)> = parse_input(input);
    let (bb, cubes) = make_grid(&cubes);

    let mut examined = bb.grid(false);
    let mut to_examine = Vec::with_capacity(bb.size());
    to_examine.push((bb.xmin, bb.ymin, bb.zmin));
    debug_assert!(!cubes[bb.idx(*to_examine.last().unwrap())]);

    let mut faces = 0;

    // flood
    while let Some(cube) = to_examine.pop() {
        if !examined[bb.idx(cube)] {
            examined[bb.idx(cube)] = true;
            for neighbour in neighbours(cube) {
                if !(bb.contains(neighbour)) {
                    continue; // do not examine outside the bounding box
                }
                if examined[bb.idx(neighbour)] {
                    continue; // do not search twice
                }
                if cubes[bb.idx(neighbour)] {
                    faces += 1; // count the face between the flood and the rock
                    continue;
                }
                // a new cube
                to_examine.push(neighbour)
            }
        }
    }

    faces
}
#[derive(Debug)]
struct BoundingBox {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
}
impl BoundingBox {
    fn containing<I, P>(mut pts: I) -> Self
    where
        I: Iterator<Item = P>,
        P: Borrow<(isize, isize, isize)>,
    {
        let binding = pts
            .next()
            .expect("Cannot make a bounding box from an empty iterator");
        let pt = binding.borrow();
        let (xmin, xmax, ymin, ymax, zmin, zmax) = pts.fold(
            (pt.0, pt.0 + 1, pt.1, pt.1 + 1, pt.2, pt.2 + 1),
            |(xmin, xmax, ymin, ymax, zmin, zmax), v| {
                let (x, y, z) = v.borrow();
                (
                    xmin.min(*x),
                    xmax.max(x + 1),
                    ymin.min(*y),
                    ymax.max(y + 1),
                    zmin.min(*z),
                    zmax.max(z + 1),
                )
            },
        );
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }

    const fn expanded(self, border: isize) -> Self {
        Self {
            xmin: self.xmin - border,
            xmax: self.xmax + border,
            ymin: self.ymin - border,
            ymax: self.ymax + border,
            zmin: self.zmin - border,
            zmax: self.zmax + border,
        }
    }
    const fn contains(&self, (x, y, z): (isize, isize, isize)) -> bool {
        self.xmin <= x
            && x < self.xmax
            && self.ymin <= y
            && y < self.ymax
            && self.zmin <= z
            && z < self.zmax
    }
    const fn size(&self) -> usize {
        ((self.xmax - self.xmin) * (self.ymax - self.ymin) * (self.zmax - self.zmin)) as _
    }
    fn grid<T>(&self, fill: T) -> Vec<T>
    where
        T: Clone,
    {
        vec![fill; self.size()]
    }
    const fn idx(&self, (x, y, z): (isize, isize, isize)) -> usize {
        (((x - self.xmin) * (self.ymax - self.ymin) + (y - self.ymin)) * (self.zmax - self.zmin)
            + (z - self.zmin)) as _
    }
}

fn make_grid(cubes: &[(isize, isize, isize)]) -> (BoundingBox, Vec<bool>) {
    let bb = BoundingBox::containing(cubes.iter()).expanded(1);
    let mut grid = bb.grid(false);
    for cube in cubes {
        grid[bb.idx(*cube)] = true;
    }
    (bb, grid)
}
