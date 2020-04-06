// arithmetic.rs
// Offers addition/subtraction abilities for a Hash

use super::hash;

use hash::Point;
use std::ops::{Add, Sub};

// Addition adds each value separately
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            lat: self.lat + other.lat,
            lon: self.lon + other.lon,
            time: self.time + other.time,
        }
    }
}

// Subtraction adds each value separately
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            lat: self.lat - other.lat,
            lon: self.lon - other.lon,
            time: self.time - other.time,
        }
    }
}

#[cfg(test)]
mod tests {
    // Franco: now this super will see high_or_low_time because it's a submodule
    use super::*;

    #[test]
    fn add_point() {
        // Sherman, TX
        let swi: Point = Point {
            lat: 33.6472022,
            lon: -96.5987648,
            time: 1585512888000000000,
        };
        // Providence, RI
        let pvd: Point = Point {
            lat: 41.8269387,
            lon: -71.4017563,
            time: 1586182020000000000,
        };
        let point: Point = swi + pvd;
        // 75.4741409
        assert!(point.lat > 75.47);
        assert!(point.lat < 75.48);
        // -168.0005211
        assert!(point.lon > -169.0);
        assert!(point.lon < -168.0);
        // 3171694908000000000
        assert!(point.time == 3171694908000000000);
    }

    #[test]
    fn sub_point() {
        // Sherman, TX
        let swi: Point = Point {
            lat: 33.6472022,
            lon: -96.5987648,
            time: 1585512888000000000,
        };
        // Providence, RI
        let pvd: Point = Point {
            lat: 41.8269387,
            lon: -71.4017563,
            time: 1586182020000000000,
        };
        let point: Point = swi - pvd;
        // -8.1797365
        assert!(point.lat > -8.20);
        assert!(point.lat < -8.17);
        // -25.1970085
        assert!(point.lon > -25.20);
        assert!(point.lon < -25.19);
        // -669132000000000
        assert!(point.time == -669132000000000);
    }
}
