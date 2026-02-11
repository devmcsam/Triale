use crate::errors::AppError;
use crate::point::Point;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub const fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
    pub const fn zero() -> Self {
        Self::new(Point::zero(), Point::zero(), Point::zero())
    }
    pub const fn one() -> Self {
        Self::new(Point::one(), Point::one(), Point::one())
    }
    pub const fn splat_recursive(size: f64) -> Self {
        Self::new(Point::splat(size), Point::splat(size), Point::splat(size))
    }
    pub const fn splat(point: Point) -> Self {
        Self::new(point, point, point)
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle[{}, {}, {}]", self.a, self.b, self.c)
    }
}

impl From<(Point, Point, Point)> for Triangle {
    fn from((first, second, third): (Point, Point, Point)) -> Self {
        Self::new(first, second, third)
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for Triangle {
    fn from((first, second, third, fourth, fifth, sixth): (f64, f64, f64, f64, f64, f64)) -> Self {
        Self::new(
            Point::new(first, second),
            Point::new(third, fourth),
            Point::new(fifth, sixth),
        )
    }
}

impl From<[Point; 3]> for Triangle {
    fn from([first, second, third]: [Point; 3]) -> Self {
        Self::new(first, second, third)
    }
}

impl From<[f64; 6]> for Triangle {
    fn from([first, second, third, fourth, fifth, sixth]: [f64; 6]) -> Self {
        Self::new(
            Point::new(first, second),
            Point::new(third, fourth),
            Point::new(fifth, sixth),
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum TriangleCreateError {
    InvalidPointCount { got: usize },
    InvalidFormat { got: String, example: String },
    DuplicatePoint { point: Point },
}

impl Display for TriangleCreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPointCount { got } => {
                write!(f, "Invalid point count: got {got}, expected 3")
            }
            Self::InvalidFormat { got, example } => {
                write!(
                    f,
                    "Invalid triangle format: got '{got}', expected '{example}'"
                )
            }
            Self::DuplicatePoint { point } => {
                write!(f, "Duplicate point: {point} is used more than once")
            }
        }
    }
}

impl Error for TriangleCreateError {}

/// Errors encountered when points do not form a valid triangle.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DegenerateTriangleError {
    Collinear {
        a: Point,
        b: Point,
        c: Point,
    },
    InequalityViolation {
        side_a: f64,
        side_b: f64,
        side_c: f64,
    },
}

impl Display for DegenerateTriangleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Collinear { a, b, c } => {
                write!(f, "Points {a}, {b}, and {c} are collinear")
            }
            Self::InequalityViolation {
                side_a,
                side_b,
                side_c,
            } => {
                write!(
                    f,
                    "Triangle inequality violated: sides {side_a:.4}, {side_b:.4}, {side_c:.4} cannot form a triangle (one side ≥ sum of others)"
                )
            }
        }
    }
}

impl Error for DegenerateTriangleError {}

pub fn check_duplicate_points(points: &[Point; 3]) -> Result<(), TriangleCreateError> {
    if points[0] == points[1] {
        return Err(TriangleCreateError::DuplicatePoint { point: points[0] });
    }
    if points[1] == points[2] {
        return Err(TriangleCreateError::DuplicatePoint { point: points[1] });
    }
    if points[0] == points[2] {
        return Err(TriangleCreateError::DuplicatePoint { point: points[0] });
    }
    Ok(())
}

pub fn check_collinear(points: &[Point; 3]) -> Result<(), DegenerateTriangleError> {
    let a = points[0];
    let b = points[1];
    let c = points[2];

    let ab = b - a;
    let ac = c - a;
    let cross_product = ab.cross(ac).abs();

    if cross_product < 1e-10 {
        return Err(DegenerateTriangleError::Collinear { a, b, c });
    }
    Ok(())
}

pub fn is_valid_triangle(
    side_a: f64,
    side_b: f64,
    side_c: f64,
) -> Result<(), DegenerateTriangleError> {
    // Triangle inequality: every side must be less than sum of other two.
    if side_a + side_b > side_c && side_a + side_c > side_b && side_b + side_c > side_a {
        return Ok(());
    }
    Err(DegenerateTriangleError::InequalityViolation {
        side_a,
        side_b,
        side_c,
    })
}

pub fn build_triangle(points: [Point; 3]) -> Result<Triangle, AppError> {
    check_duplicate_points(&points)?;
    check_collinear(&points)?;

    let side_a = points[1].distance_to(points[2]);
    let side_b = points[0].distance_to(points[2]);
    let side_c = points[0].distance_to(points[1]);
    is_valid_triangle(side_a, side_b, side_c)?;

    Ok(Triangle::new(points[0], points[1], points[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_points() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(0.0, 0.0);
        let p3 = Point::new(1.0, 1.0);

        assert!(check_duplicate_points(&[p1, p2, p3]).is_err());
        assert!(check_duplicate_points(&[p1, p3, p2]).is_err());
        assert!(check_duplicate_points(&[p3, p1, p2]).is_err());
        assert!(check_duplicate_points(&[p1, p3, Point::new(2.0, 2.0)]).is_ok());
    }

    #[test]
    fn test_near_duplicate_points() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1e-18, 1e-18);
        let p3 = Point::new(1.0, 1.0);

        // These are currently NOT considered duplicates because we use exact equality
        assert!(check_duplicate_points(&[p1, p2, p3]).is_ok());

        // But build_triangle should fail due to collinearity or triangle inequality
        assert!(build_triangle([p1, p2, p3]).is_err());
    }

    #[test]
    fn test_collinear_points() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 0.0);
        let p3 = Point::new(2.0, 0.0);
        assert!(check_collinear(&[p1, p2, p3]).is_err());

        let p4 = Point::new(0.0, 1.0);
        assert!(check_collinear(&[p1, p2, p4]).is_ok());

        // Near collinear
        let p5 = Point::new(2.0, 1e-11);
        assert!(
            check_collinear(&[p1, p2, p5]).is_err(),
            "Should be considered collinear with 1e-11 offset"
        );

        let p6 = Point::new(2.0, 1e-9);
        assert!(
            check_collinear(&[p1, p2, p6]).is_ok(),
            "Should NOT be considered collinear with 1e-9 offset"
        );
    }

    #[test]
    fn test_triangle_inequality() {
        // Valid
        assert!(is_valid_triangle(3.0, 4.0, 5.0).is_ok());
        // Degenerate (sum of two equals third)
        assert!(is_valid_triangle(1.0, 1.0, 2.0).is_err());
        // Impossible
        assert!(is_valid_triangle(1.0, 1.0, 3.0).is_err());
    }

    #[test]
    fn test_build_triangle() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 0.0);
        let p3 = Point::new(0.0, 1.0);
        assert!(build_triangle([p1, p2, p3]).is_ok());

        // Duplicates
        assert!(build_triangle([p1, p1, p3]).is_err());
        // Collinear
        assert!(build_triangle([p1, p2, Point::new(2.0, 0.0)]).is_err());
    }

    #[test]
    fn test_small_valid_triangle() {
        let s = 1e-4; // s^2 = 1e-8 > 1e-10
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(s, 0.0);
        let p3 = Point::new(0.0, s);
        assert!(build_triangle([p1, p2, p3]).is_ok());

        let s2 = 1e-6; // s2^2 = 1e-12 < 1e-10
        let p4 = Point::new(s2, 0.0);
        let p5 = Point::new(0.0, s2);
        assert!(
            build_triangle([p1, p4, p5]).is_err(),
            "Should be rejected as too small/collinear"
        );
    }
}
