use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IVec2 {
    pub x: i64,
    pub y: i64,
}

impl IVec2 {
    pub fn new(x: i64, y: i64) -> IVec2 {
        IVec2 { x, y }
    }

    pub fn dot(lhs: &IVec2, rhs: &IVec2) -> i64 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn cross(lhs: &IVec2, rhs: &IVec2) -> IVec2 {
        IVec2 {
            x: lhs.x * rhs.y - lhs.y * rhs.x,
            y: 0,
        }
    }

    pub fn distance(lhs: &IVec2, rhs: &IVec2) -> f64 {
        (((lhs.x - rhs.x).pow(2) + (lhs.y - rhs.y).pow(2)) as f64).sqrt()
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }

    pub fn distance_to(&self, other: &IVec2) -> f64 {
        Self::distance(self, other)
    }

    pub fn rect_area(lhs: &IVec2, rhs: &IVec2) -> i64 {
        (i64::abs(lhs.x - rhs.x) + 1) * (i64::abs(lhs.y - rhs.y) + 1)
    }
}

#[derive(Debug)]
pub struct IRect {
    pub a: IVec2,
    pub b: IVec2,
    pub c: IVec2,
    pub d: IVec2,
}

impl Sub for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl IRect {
    pub fn area(&self) -> i64 {
        // doesn't work
        // let ab = self.b - self.a;
        // let ad = self.d - self.a;
        // let cross = IVec2::cross(&ab, &ad);
        //
        // cross.magnitude() as i64
        (i64::abs(self.a.x - self.c.x) + 1) * (i64::abs(self.a.y - self.c.y) + 1)
    }
    pub fn contains(&self, point: &IVec2) -> bool {
        let ab = self.b - self.a;
        let am = *point - self.a;
        let bc = self.c - self.b;
        let bm = *point - self.b;

        (0 <= IVec2::dot(&ab, &am))
            && (IVec2::dot(&ab, &am) < IVec2::dot(&ab, &ab))
            && (0 < IVec2::dot(&bc, &bm))
            && (IVec2::dot(&bc, &bm) < IVec2::dot(&bc, &bc))
    }
}

#[cfg(test)]
mod tests {
    use crate::{IRect, IVec2};

    #[test]
    fn test_sub() {
        let x = IVec2::new(-1, 2);
        let y = IVec2::new(1, 2);
        assert_eq!(y - x, IVec2::new(2, 0));
    }

    #[test]
    fn test_cross() {
        let x = IVec2::new(2, 0);
        let y = IVec2::new(0, -1);
        assert_eq!(IVec2::cross(&x, &y), IVec2::new(-2, 0));
    }

    #[test]
    fn test_magnitude() {
        let x = IVec2::new(-2, 0);
        assert_eq!(x.magnitude(), 2.0);
    }

    #[test]
    fn test_contains() {
        let rectangle = IRect {
            a: IVec2 { x: 50, y: 0 },
            b: IVec2 { x: 0, y: 20 },
            c: IVec2 { x: 10, y: 50 },
            d: IVec2 { x: 60, y: 30 },
        };
        let mut count = 0;
        for y in (0..50).rev() {
            for x in 0..65 {
                let point = IVec2::new(x, y);
                if rectangle.contains(&point) {
                    count += 1;
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        assert_eq!(count, 1729)
    }
}
