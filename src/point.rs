use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub const fn splat(number: f64) -> Self {
        Self::new(number, number)
    }
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub const fn one() -> Self {
        Self::new(1.0, 1.0)
    }

    /// Euclidean distance to another point
    pub fn distance_to(self, other: Self) -> f64 {
        (other.x - self.x).hypot(other.y - self.y)
    }

    /// Dot product treating points as 2D vectors.
    pub fn dot(self, other: Self) -> f64 {
        self.x.mul_add(other.x, self.y * other.y)
    }

    /// 2D cross product (z-component of the 3D cross product).
    pub fn cross(self, other: Self) -> f64 {
        self.x.mul_add(other.y, -(self.y * other.x))
    }

    /// Squared Euclidean length
    pub fn length_sq(self) -> f64 {
        self.x.mul_add(self.x, self.y * self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Normalize 0.0 and -0.0 so they have the same hash
        let x = if self.x == 0.0 { 0.0 } else { self.x };
        let y = if self.y == 0.0 { 0.0 } else { self.y };
        x.to_bits().hash(state);
        y.to_bits().hash(state);
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<f64> for Point {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f64> for Point {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<f64> for Point {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl SubAssign<f64> for Point {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<[f64; 2]> for Point {
    fn from([x, y]: [f64; 2]) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PointCreateError {
    InvalidFormat { got: String, example: String },
    TooManyPoints { got: usize, expected: u8 },
    TooFewPoints { got: usize, expected: u8 },
}

impl Display for PointCreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat { got, example } => {
                write!(f, "Invalid point format: got '{got}', expected '{example}'")
            }
            Self::TooManyPoints { got, expected } => {
                write!(f, "Too many points: got {got}, expected {expected}")
            }
            Self::TooFewPoints { got, expected } => {
                write!(f, "Too few points: got {got}, expected {expected}")
            }
        }
    }
}

impl Error for PointCreateError {}

pub fn to_point(input: &str) -> Result<Point, PointCreateError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(PointCreateError::InvalidFormat {
            got: String::new(),
            example: "1.0,2.0".to_string(),
        });
    }

    let parts: Vec<&str> = trimmed.split(',').map(str::trim).collect();

    match parts.len().cmp(&2) {
        std::cmp::Ordering::Less => {
            return Err(PointCreateError::TooFewPoints {
                got: parts.len(),
                expected: 2,
            });
        }
        std::cmp::Ordering::Greater => {
            return Err(PointCreateError::TooManyPoints {
                got: parts.len(),
                expected: 2,
            });
        }
        std::cmp::Ordering::Equal => {}
    }

    let x_raw = parts[0];
    let y_raw = parts[1];

    let x = x_raw
        .parse::<f64>()
        .map_err(|_| PointCreateError::InvalidFormat {
            got: x_raw.to_string(),
            example: "x: a valid decimal value e.g. 1.0".to_string(),
        })?;
    let x = validate_finite(x, "x", x_raw)?;

    let y = y_raw
        .parse::<f64>()
        .map_err(|_| PointCreateError::InvalidFormat {
            got: y_raw.to_string(),
            example: "y: a valid decimal value e.g. 2.0".to_string(),
        })?;
    let y = validate_finite(y, "y", y_raw)?;

    Ok(Point::new(x, y))
}

fn validate_finite(value: f64, label: &str, raw: &str) -> Result<f64, PointCreateError> {
    if value.is_nan() {
        return Err(PointCreateError::InvalidFormat {
            got: raw.to_string(),
            example: format!("{label}: a finite decimal value (NaN is not allowed)"),
        });
    }
    if value.is_infinite() {
        return Err(PointCreateError::InvalidFormat {
            got: raw.to_string(),
            example: format!("{label}: a finite decimal value (infinity is not allowed)"),
        });
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point_arithmetic() {
        let p1 = Point::new(10.0, 20.0);
        let p2 = Point::new(5.0, 5.0);

        assert_eq!(p1 + p2, Point::new(15.0, 25.0));
        assert_eq!(p1 - p2, Point::new(5.0, 15.0));
        assert_eq!(p1 * 2.0, Point::new(20.0, 40.0));
        assert_eq!(p1 / 2.0, Point::new(5.0, 10.0));
        assert_eq!(-p1, Point::new(-10.0, -20.0));

        let mut p3 = p1;
        p3 += p2;
        assert_eq!(p3, Point::new(15.0, 25.0));
        p3 -= p2;
        assert_eq!(p3, Point::new(10.0, 20.0));
        p3 *= 2.0;
        assert_eq!(p3, Point::new(20.0, 40.0));
        p3 /= 4.0;
        assert_eq!(p3, Point::new(5.0, 10.0));
    }

    #[test]
    fn test_point_geometry_helpers() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);
        assert_eq!(p2.length_sq(), 25.0);
        assert_eq!(p1.dot(p2), 0.0);
        assert_eq!(p1.cross(p2), 0.0);

        let p3 = Point::new(1.0, 0.0);
        let p4 = Point::new(0.0, 1.0);
        assert_eq!(p3.dot(p4), 0.0);
        assert_eq!(p3.cross(p4), 1.0);
    }

    #[test]
    fn test_to_point_parsing() {
        // Basic parsing
        assert_eq!(to_point("1.0, 2.0").unwrap(), Point::new(1.0, 2.0));
        assert_eq!(to_point(" -1.5 , 3.14 ").unwrap(), Point::new(-1.5, 3.14));

        // Scientific notation
        assert_eq!(to_point("1e2, 2.5e-1").unwrap(), Point::new(100.0, 0.25));

        // Errors
        assert!(to_point("").is_err());
        assert!(to_point("1.0").is_err());
        assert!(to_point("1.0, 2.0, 3.0").is_err());
        assert!(to_point("abc, 2.0").is_err());
        assert!(to_point("1.0, def").is_err());
        assert!(to_point("NaN, 1.0").is_err());
        assert!(to_point("inf, 1.0").is_err());
    }

    #[test]
    fn test_extreme_values() {
        let p1 = Point::new(1e150, 1e150);
        let p2 = Point::new(-1e150, -1e150);
        // distance_to uses hypot, should not overflow to inf easily
        assert!(p1.distance_to(p2).is_finite());

        let p_tiny = Point::new(1e-150, 1e-150);
        assert!(p_tiny.length_sq() < 1e-299);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(0.0, 0.0);
        let p3 = Point::new(-0.0, 0.0);

        assert_eq!(calculate_hash(&p1), calculate_hash(&p2));
        // Note: In IEEE 754, 0.0 == -0.0, but they have different bit patterns.
        // Our Hash implementation uses to_bits(), so 0.0 and -0.0 will have different hashes.
        // This is actually "correct" if we want to distinguish them, but might be surprising
        // due to partialeq saying they are equal.
        if p1 == p3 {
            assert_eq!(
                calculate_hash(&p1),
                calculate_hash(&p3),
                "Hash must match for equal points (0.0 and -0.0)"
            );
        }
    }
}
