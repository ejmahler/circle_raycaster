use std::ops::{Generator, GeneratorState};
use cgmath::{Point2, Vector2, BaseNum};
use cgmath::num_traits::Signed;

// Simple 90 degree rotations
trait CardinalRotation<T> {
    fn rotate_90(&self) -> Self;
    fn rotate_180(&self) -> Self;
    fn rotate_270(&self) -> Self;
}
impl<T: Copy + Signed> CardinalRotation<T> for Vector2<T> {
    fn rotate_90(&self) -> Self {
        Self { x: self.y.neg(), y: self.x }
    }
    fn rotate_180(&self) -> Self {
        Self { x: self.x.neg(), y: self.y.neg() }
    }
    fn rotate_270(&self) -> Self {
        Self { x: self.y, y: self.x.neg() }
    }
}

// Rotations in 45 degree increments
#[derive(Clone, Copy, Debug)]
enum OctantRotation {
    Octant0,
    Octant45,
    Octant90,
    Octant135,
    Octant180,
    Octant225,
    Octant270,
    Octant315,
}

// Represents a Vector2, restricted t othe case where x>=0, y>=0, and x>=y
// From here, we can rotate to all other octants through swizzling and negating and etc, and convert other vectors t othis octsnt
// Apparently a lot of algorithms on pixels are simpler if you restrict thme to octant zero
#[derive(Clone, Copy, Debug)]
struct OctantZeroVector<T>(Vector2<T>);
impl<T: Copy + Signed + BaseNum> OctantZeroVector<T> {
    fn rotate_to(&self, to: OctantRotation) -> Vector2<T> {
        let OctantZeroVector(inner_vec) = self;

        match to {
            OctantRotation::Octant0 => *inner_vec,
            OctantRotation::Octant45 => inner_vec.yx(),
            OctantRotation::Octant90 => inner_vec.rotate_90(),
            OctantRotation::Octant135 => inner_vec.yx().rotate_90(),
            OctantRotation::Octant180 => inner_vec.rotate_180(),
            OctantRotation::Octant225 => inner_vec.yx().rotate_180(),
            OctantRotation::Octant270 => inner_vec.rotate_270(),
            OctantRotation::Octant315 => inner_vec.yx().rotate_270(),
        }
    }
    fn unrotate_from(from: Vector2<T>) -> (Self, OctantRotation) {
        if !from.y.is_negative() {
            if !from.x.is_negative() {
                if from.x >= from.y {
                    (OctantZeroVector(from), OctantRotation::Octant0)
                } else {
                    (OctantZeroVector(from.yx()), OctantRotation::Octant45)
                }
            } else {
                if from.y >= from.x.neg() {
                    (OctantZeroVector(from.rotate_270()), OctantRotation::Octant90)
                } else {
                    (OctantZeroVector(from.rotate_270().yx()), OctantRotation::Octant135)
                }
            }
        } else {
            if from.x.is_negative() {
                if from.x.neg() >= from.y.neg() {
                    (OctantZeroVector(from.rotate_180()), OctantRotation::Octant180)
                } else {
                    (OctantZeroVector(from.rotate_180().yx()), OctantRotation::Octant225)
                }
            } else {
                if from.y.neg() >= from.x {
                    (OctantZeroVector(from.rotate_90()), OctantRotation::Octant270)
                } else {
                    (OctantZeroVector(from.rotate_90().yx()), OctantRotation::Octant315)
                }
            }
        }
    }
    fn try_from(from: &Vector2<T>) -> Option<Self> {
        if !from.y.is_negative() && !from.x.is_negative() && from.x >= from.y {
            Some(OctantZeroVector(*from))
        } else {
            None
        }
    }
    fn from_unchecked(x: T, y: T) -> Self {
        OctantZeroVector(Vector2::new(x, y))
    }
    fn x(&self) -> T {
        self.0.x
    }
    fn y(&self) -> T {
        self.0.y
    }
}


pub fn iter_pixels_in_line(from: Point2<i32>, to: Point2<i32>) -> impl Iterator<Item=Point2<i32>> {
    generator_to_iterator(move || {
        let (octant_offset, octant_rotation) = OctantZeroVector::unrotate_from(to - from);

        let mut current_error = 2 * octant_offset.y() - octant_offset.x();
        let mut y = 0;

        for x in 0..=octant_offset.x() {
            yield from + OctantZeroVector::from_unchecked(x, y).rotate_to(octant_rotation);

            if !current_error.is_negative() {
                y += 1;
                current_error -= 2 * octant_offset.x();
            }
            current_error += 2 * octant_offset.y();
        }
    })
}

pub fn iter_pixels_in_circle(radius: i32, center: Point2<i32>) -> impl Iterator<Item=Point2<i32>> {
    generator_to_iterator(move || {
        let octant_pixels: Vec<OctantZeroVector<i32>> = CirclePixelOctant::new(radius).collect();
        let reflected_end_index = octant_pixels.last().map_or(0, |OctantZeroVector(p)|
            if p.x == p.y { 
                octant_pixels.len() - 2
            } else {
                octant_pixels.len() - 1
            }
        );
        
        // first quadrant
        for i in 0..octant_pixels.len() {
            yield center + octant_pixels[i].rotate_to(OctantRotation::Octant0);
        }
        for i in 0..reflected_end_index {
            yield center + octant_pixels[reflected_end_index - i].rotate_to(OctantRotation::Octant45);
        }

        // second quadrant
        for i in 0..octant_pixels.len() {
            yield center + octant_pixels[i].rotate_to(OctantRotation::Octant90);
        }
        for i in 0..reflected_end_index {
            yield center + octant_pixels[reflected_end_index - i].rotate_to(OctantRotation::Octant135);
        }

        // third quadrant
        for i in 0..octant_pixels.len() {
            yield center + octant_pixels[i].rotate_to(OctantRotation::Octant180);
        }
        for i in 0..reflected_end_index {
            yield center + octant_pixels[reflected_end_index - i].rotate_to(OctantRotation::Octant225);
        }

        // fourth quadrant
        for i in 0..octant_pixels.len() {
            yield center + octant_pixels[i].rotate_to(OctantRotation::Octant270)
        }
        for i in 0..reflected_end_index {
            yield center + octant_pixels[reflected_end_index - i].rotate_to(OctantRotation::Octant315)
        }
    })
}

struct CirclePixelOctant {
    p: Option<OctantZeroVector<i32>>,
    radius_squared: i32,
}
impl CirclePixelOctant {
    fn new(radius: i32) -> Self {
        Self { 
            p: OctantZeroVector::try_from(&Vector2{ x: radius, y: 0 }), 
            radius_squared: (radius + 1) * radius,
        }
    }
}
impl Iterator for CirclePixelOctant {
    type Item = OctantZeroVector<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_val = self.p;
        if let Some(OctantZeroVector(mut vec)) = old_val {
            let radius_error = vec.x*vec.x + vec.y*vec.y - self.radius_squared;
            if 2 * (radius_error - 1) + 1 > 0 {
               vec.x -= 1;
            }
            vec.y += 1;

            // Once we pass the y=x line, this will return None, so the next iteration will return None
            self.p = OctantZeroVector::try_from(&vec);
        }
        old_val
    }
}

fn generator_to_iterator<G>(g: G) -> impl Iterator<Item = G::Yield>
where G: Generator<Return = ()> {
    struct It<G>(G);

    impl<G: Generator<Return = ()>> Iterator for It<G> {
        type Item = G::Yield;

        fn next(&mut self) -> Option<Self::Item> {
            match unsafe { self.0.resume() } {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(()) => None,
            }
        }
    }

    It(g)
}