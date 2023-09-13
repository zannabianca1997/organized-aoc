use std::ops::{Mul, Sub};

use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<X, Y = X> {
    pub minx: X,
    pub maxx: X,
    pub miny: Y,
    pub maxy: Y,
}
impl<X, Y> Rect<X, Y>
where
    X: Ord,
    Y: Ord,
{
    /// Check if `self` is a super-rect of `other`
    pub fn is_super(&self, other: &Self) -> bool {
        self.minx <= other.minx
            && other.maxx <= self.maxx
            && self.miny <= other.miny
            && other.maxy <= self.maxy
    }
    /// Check if `self` contains (x,y)
    pub fn contains(&self, x: X, y: Y) -> bool {
        self.minx <= x && x < self.maxx && self.miny <= y && y < self.maxy
    }
    /// Check if `self` collide with `other`
    pub fn collide(&self, other: &Self) -> bool {
        self.minx < other.maxx
            && other.minx < self.maxx
            && self.miny < other.maxy
            && other.miny < self.maxy
    }
}
impl<X, Y> Rect<X, Y>
where
    X: Ord + Clone,
    Y: Ord + Clone,
{
    /// Cut other in smaller rectangles, each one inside or outside `self`
    pub fn cut(&self, other: Self) -> (ArrayVec<Self, 4>, Option<Self>) {
        // some trivial cases
        if !self.collide(&other) {
            let mut outside = ArrayVec::new();
            outside.push(other);
            return (outside, None);
        }
        // general splitting algorithm
        let Rect {
            minx,
            maxx,
            miny,
            maxy,
        } = other;

        // find where `self` split `other`
        let mut outside = ArrayVec::new();
        let inside;
        match (
            self.minx > minx,
            self.maxx < maxx,
            self.miny > miny,
            self.maxy < maxy,
        ) {
            // full case, 3x3 grid
            // divide the extern in 4 rects
            (true, true, true, true) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny: miny.clone(),
                    maxy: self.miny.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny,
                    maxy,
                });
                inside = Some(self.clone())
            }
            // 1 external side
            // use 3 rects for the extern
            (true, true, true, false) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny: self.miny.clone(),
                    maxy,
                })
            }
            (true, true, false, true) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny,
                    maxy: self.maxy.clone(),
                })
            }
            (true, false, true, true) => {
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: self.miny.clone(),
                    maxy: self.maxy.clone(),
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx,
                    miny: self.miny.clone(),
                    maxy: self.maxy.clone(),
                })
            }
            (false, true, true, true) => {
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: self.miny.clone(),
                    maxy: self.maxy.clone(),
                });
                inside = Some(Rect {
                    minx,
                    maxx: self.maxx.clone(),
                    miny: self.miny.clone(),
                    maxy: self.maxy.clone(),
                })
            }
            // 2 opposite external sides
            // use 2 rects for the extern
            (true, true, false, false) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx: self.maxx.clone(),
                    miny,
                    maxy,
                })
            }
            (false, false, true, true) => {
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                inside = Some(Rect {
                    minx,
                    maxx,
                    miny: self.miny.clone(),
                    maxy: self.maxy.clone(),
                })
            }
            // single corner cut
            // use 2 rects for the extern
            (true, false, true, false) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx,
                    miny: self.miny.clone(),
                    maxy,
                })
            }
            (false, true, false, true) => {
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: self.maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                inside = Some(Rect {
                    minx,
                    maxx: self.maxx.clone(),
                    miny,
                    maxy: self.maxy.clone(),
                })
            }
            (true, false, false, true) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                outside.push(Rect {
                    minx: self.minx.clone(),
                    maxx: maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx,
                    miny,
                    maxy: self.maxy.clone(),
                })
            }
            (false, true, true, false) => {
                outside.push(Rect {
                    miny,
                    maxy: self.miny.clone(),
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                });
                outside.push(Rect {
                    miny: self.miny.clone(),
                    maxy: maxy.clone(),
                    minx: self.maxx.clone(),
                    maxx,
                });
                inside = Some(Rect {
                    miny: self.miny.clone(),
                    maxy,
                    minx,
                    maxx: self.maxx.clone(),
                })
            }
            // Cut in half
            // Use a single rect
            (true, false, false, false) => {
                outside.push(Rect {
                    minx,
                    maxx: self.minx.clone(),
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                inside = Some(Rect {
                    minx: self.minx.clone(),
                    maxx,
                    miny,
                    maxy,
                })
            }
            (false, true, false, false) => {
                outside.push(Rect {
                    minx: self.maxx.clone(),
                    maxx,
                    miny: miny.clone(),
                    maxy: maxy.clone(),
                });
                inside = Some(Rect {
                    minx,
                    maxx: self.maxx.clone(),
                    miny,
                    maxy,
                })
            }
            (false, false, true, false) => {
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny,
                    maxy: self.miny.clone(),
                });
                inside = Some(Rect {
                    minx,
                    maxx,
                    miny: self.miny.clone(),
                    maxy,
                })
            }
            (false, false, false, true) => {
                outside.push(Rect {
                    minx: minx.clone(),
                    maxx: maxx.clone(),
                    miny: self.maxy.clone(),
                    maxy,
                });
                inside = Some(Rect {
                    minx,
                    maxx,
                    miny,
                    maxy: self.maxy.clone(),
                })
            }
            // `self` contains `other`
            (false, false, false, false) => {
                inside = Some(Rect {
                    minx,
                    maxx,
                    miny,
                    maxy,
                });
            }
        }

        (outside, inside)
    }
}

impl<X, Y> Rect<X, Y>
where
    X: Sub,
    Y: Sub,
    X::Output: Mul<Y::Output>,
{
    pub fn area(self) -> <X::Output as Mul<Y::Output>>::Output {
        let Rect {
            minx,
            maxx,
            miny,
            maxy,
        } = self;
        (maxx - minx) * (maxy - miny)
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::Rect;

    fn rects() -> impl Iterator<Item = Rect<usize>> {
        (0..3).flat_map(|minx| {
            (minx + 1..4).flat_map(move |maxx| {
                (0..3).flat_map(move |miny| {
                    (miny + 1..4).map(move |maxy| Rect {
                        minx,
                        maxx,
                        miny,
                        maxy,
                    })
                })
            })
        })
    }

    #[test]
    fn cut() {
        for cutter in rects() {
            for cutted in rects() {
                //dbg!(cutted, cutter);
                let (outside, inside) = cutter.cut(cutted);
                assert!(outside.iter().all(|o| !cutter.collide(o)));
                assert!(outside.iter().all(|o| cutted.is_super(o)));
                for i in 0..outside.len() {
                    for j in i + 1..outside.len() {
                        assert!(!outside[i].collide(&outside[j]))
                    }
                }
                if let Some(inside) = inside {
                    assert!(cutter.is_super(&inside));
                    assert!(cutted.is_super(&inside))
                }
                assert_eq!(
                    outside.iter().map(|r| r.area()).sum::<usize>()
                        + inside.map(|r| r.area()).unwrap_or(0),
                    cutted.area()
                )
            }
        }
    }
}
