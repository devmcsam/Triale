use crate::point::Point;
use crate::triangle::Triangle;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SideClassification {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Display for SideClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equilateral => write!(f, "Equilateral"),
            Self::Isosceles => write!(f, "Isosceles"),
            Self::Scalene => write!(f, "Scalene"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum AngleClassification {
    Acute,
    Right,
    Obtuse,
}

impl Display for AngleClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acute => write!(f, "Acute"),
            Self::Right => write!(f, "Right"),
            Self::Obtuse => write!(f, "Obtuse"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TriangleSummary {
    pub vertex_a: Point,
    pub vertex_b: Point,
    pub vertex_c: Point,
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
    pub angle_a_rad: f64,
    pub angle_b_rad: f64,
    pub angle_c_rad: f64,
    pub perimeter: f64,
    pub semi_perimeter: f64,
    pub area: f64,
    pub side_class: SideClassification,
    pub angle_class: AngleClassification,
    pub centroid: Point,
    pub incenter: Point,
    pub circumcenter: Point,
    pub orthocenter: Point,
    pub nine_point_center: Point,
    pub inradius: f64,
    pub circumradius: f64,
    pub nine_point_radius: f64,
    pub median_a: f64,
    pub median_b: f64,
    pub median_c: f64,
    pub altitude_a: f64,
    pub altitude_b: f64,
    pub altitude_c: f64,
    pub bisector_a: f64,
    pub bisector_b: f64,
    pub bisector_c: f64,
}

pub fn approx_eq(a: f64, b: f64) -> bool {
    let abs_tol = 1e-9;
    let rel_tol = 1e-9;
    let diff = (a - b).abs();
    diff <= abs_tol || diff <= rel_tol * a.abs().max(b.abs())
}

pub fn angle_from_sides(opposite: f64, adj1: f64, adj2: f64) -> f64 {
    let numerator = opposite.mul_add(-opposite, adj1.mul_add(adj1, adj2 * adj2));
    let denominator = 2.0 * adj1 * adj2;
    let cos_val = (numerator / denominator).clamp(-1.0, 1.0);
    cos_val.acos()
}

pub fn median_length(opposite_side: f64, adj1: f64, adj2: f64) -> f64 {
    let two_b_squared = 2.0 * adj1 * adj1;
    let two_c_squared = 2.0 * adj2 * adj2;
    let a_sq = opposite_side * opposite_side;
    0.5 * (two_b_squared + two_c_squared - a_sq).max(0.0).sqrt()
}

pub fn altitude_from_area(area: f64, opposite_side: f64) -> f64 {
    2.0 * area / opposite_side
}

pub fn bisector_length(adj1: f64, adj2: f64, angle_rad: f64) -> f64 {
    let half_cos = (angle_rad / 2.0).cos();
    (2.0 * adj1 * adj2 * half_cos) / (adj1 + adj2)
}

pub fn classify_sides(side_a: f64, side_b: f64, side_c: f64) -> SideClassification {
    let ab = approx_eq(side_a, side_b);
    let bc = approx_eq(side_b, side_c);
    let ac = approx_eq(side_a, side_c);
    if ab && bc && ac {
        SideClassification::Equilateral
    } else if ab || bc || ac {
        SideClassification::Isosceles
    } else {
        SideClassification::Scalene
    }
}

pub fn classify_angles(angle_a: f64, angle_b: f64, angle_c: f64) -> AngleClassification {
    let pi_half = std::f64::consts::PI / 2.0;
    if approx_eq(angle_a, pi_half) || approx_eq(angle_b, pi_half) || approx_eq(angle_c, pi_half) {
        AngleClassification::Right
    } else if angle_a > pi_half || angle_b > pi_half || angle_c > pi_half {
        AngleClassification::Obtuse
    } else {
        AngleClassification::Acute
    }
}

pub fn circumcenter(a: Point, b: Point, c: Point) -> Point {
    let d = 2.0
        * c.x
            .mul_add(a.y - b.y, a.x.mul_add(b.y - c.y, b.x * (c.y - a.y)));
    let a_sq = a.length_sq();
    let b_sq = b.length_sq();
    let c_sq = c.length_sq();

    let ux = c_sq.mul_add(a.y - b.y, a_sq.mul_add(b.y - c.y, b_sq * (c.y - a.y))) / d;
    let uy = c_sq.mul_add(b.x - a.x, a_sq.mul_add(c.x - b.x, b_sq * (a.x - c.x))) / d;
    Point::new(ux, uy)
}

pub fn incenter(pa: Point, pb: Point, pc: Point, side_a: f64, side_b: f64, side_c: f64) -> Point {
    let perimeter = side_a + side_b + side_c;
    let x = side_c.mul_add(pc.x, side_a.mul_add(pa.x, side_b * pb.x)) / perimeter;
    let y = side_c.mul_add(pc.y, side_a.mul_add(pa.y, side_b * pb.y)) / perimeter;
    Point::new(x, y)
}

pub fn centroid(a: Point, b: Point, c: Point) -> Point {
    (a + b + c) / 3.0
}

pub fn orthocenter(a: Point, b: Point, c: Point, circumference: Point) -> Point {
    let g = centroid(a, b, c);
    (g * 3.0) - (circumference * 2.0)
}

pub fn nine_point_center(circumference: Point, orthogonal: Point) -> Point {
    (circumference + orthogonal) / 2.0
}

pub fn compute_summary(tri: &Triangle) -> TriangleSummary {
    let a = tri.a;
    let b = tri.b;
    let c = tri.c;

    let side_a = b.distance_to(c);
    let side_b = a.distance_to(c);
    let side_c = a.distance_to(b);

    let perimeter = side_a + side_b + side_c;
    let s = perimeter / 2.0;

    // this has better accuracy with needle like triangles than regular formula
    let area = 0.5 * (b - a).cross(c - a).abs();

    let angle_a_rad = angle_from_sides(side_a, side_b, side_c);
    let angle_b_rad = angle_from_sides(side_b, side_a, side_c);
    let angle_c_rad = angle_from_sides(side_c, side_a, side_b);

    let side_class = classify_sides(side_a, side_b, side_c);
    let angle_class = classify_angles(angle_a_rad, angle_b_rad, angle_c_rad);

    let circumcenter = circumcenter(a, b, c);
    let centroid = centroid(a, b, c);
    let incenter = incenter(a, b, c, side_a, side_b, side_c);
    let orthocenter = orthocenter(a, b, c, circumcenter);
    let nine_point_center = nine_point_center(circumcenter, orthocenter);

    let inradius = area / s;
    let circumradius = (side_a * side_b * side_c) / (4.0 * area);
    let nine_point_radius = circumradius / 2.0;

    let median_a = median_length(side_a, side_b, side_c);
    let median_b = median_length(side_b, side_a, side_c);
    let median_c = median_length(side_c, side_a, side_b);

    let altitude_a = altitude_from_area(area, side_a);
    let altitude_b = altitude_from_area(area, side_b);
    let altitude_c = altitude_from_area(area, side_c);

    let bisector_a = bisector_length(side_b, side_c, angle_a_rad);
    let bisector_b = bisector_length(side_a, side_c, angle_b_rad);
    let bisector_c = bisector_length(side_a, side_b, angle_c_rad);

    TriangleSummary {
        vertex_a: a,
        vertex_b: b,
        vertex_c: c,
        side_a,
        side_b,
        side_c,
        angle_a_rad,
        angle_b_rad,
        angle_c_rad,
        perimeter,
        semi_perimeter: s,
        area,
        side_class,
        angle_class,
        centroid,
        incenter,
        circumcenter,
        orthocenter,
        nine_point_center,
        inradius,
        circumradius,
        nine_point_radius,
        median_a,
        median_b,
        median_c,
        altitude_a,
        altitude_b,
        altitude_c,
        bisector_a,
        bisector_b,
        bisector_c,
    }
}

// Display for TriangleSummary was generated by AI and then checked and reviewed by me.
impl Display for TriangleSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = 22; // label column width
        let sep = "─".repeat(60);

        writeln!(f, "\n{sep}")?;
        writeln!(f, "           ▲  TRIANGLE SUMMARY  ▲")?;
        writeln!(f, "{sep}")?;

        writeln!(f, "\n┌─ Vertices")?;
        writeln!(f, "│  {:>w$}  {}", "Vertex A:", self.vertex_a)?;
        writeln!(f, "│  {:>w$}  {}", "Vertex B:", self.vertex_b)?;
        writeln!(f, "│  {:>w$}  {}", "Vertex C:", self.vertex_c)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Side Lengths (opposite vertex)")?;
        writeln!(f, "│  {:>w$}  {:.10}  (BC)", "Side a:", self.side_a)?;
        writeln!(f, "│  {:>w$}  {:.10}  (AC)", "Side b:", self.side_b)?;
        writeln!(f, "│  {:>w$}  {:.10}  (AB)", "Side c:", self.side_c)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Angles")?;
        writeln!(
            f,
            "│  {:>w$}  {:.10} rad  ({:.6}°)",
            "Angle A:",
            self.angle_a_rad,
            self.angle_a_rad.to_degrees()
        )?;
        writeln!(
            f,
            "│  {:>w$}  {:.10} rad  ({:.6}°)",
            "Angle B:",
            self.angle_b_rad,
            self.angle_b_rad.to_degrees()
        )?;
        writeln!(
            f,
            "│  {:>w$}  {:.10} rad  ({:.6}°)",
            "Angle C:",
            self.angle_c_rad,
            self.angle_c_rad.to_degrees()
        )?;
        writeln!(
            f,
            "│  {:>w$}  {:.10}°",
            "Sum of angles:",
            (self.angle_a_rad + self.angle_b_rad + self.angle_c_rad).to_degrees()
        )?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Classification")?;
        writeln!(f, "│  {:>w$}  {}", "By sides:", self.side_class)?;
        writeln!(f, "│  {:>w$}  {}", "By angles:", self.angle_class)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Core Metrics")?;
        writeln!(f, "│  {:>w$}  {:.10}", "Perimeter:", self.perimeter)?;
        writeln!(
            f,
            "│  {:>w$}  {:.10}",
            "Semi-perimeter:", self.semi_perimeter
        )?;
        writeln!(f, "│  {:>w$}  {:.10}", "Area:", self.area)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Notable Centres")?;
        writeln!(
            f,
            "│  {:>w$}  ({:.8}, {:.8})",
            "Centroid:", self.centroid.x, self.centroid.y
        )?;
        writeln!(
            f,
            "│  {:>w$}  ({:.8}, {:.8})",
            "Incenter:", self.incenter.x, self.incenter.y
        )?;
        writeln!(
            f,
            "│  {:>w$}  ({:.8}, {:.8})",
            "Circumcenter:", self.circumcenter.x, self.circumcenter.y
        )?;
        writeln!(
            f,
            "│  {:>w$}  ({:.8}, {:.8})",
            "Orthocenter:", self.orthocenter.x, self.orthocenter.y
        )?;
        writeln!(
            f,
            "│  {:>w$}  ({:.8}, {:.8})",
            "Nine-point center:", self.nine_point_center.x, self.nine_point_center.y
        )?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Radii")?;
        writeln!(f, "│  {:>w$}  {:.10}", "Inradius:", self.inradius)?;
        writeln!(f, "│  {:>w$}  {:.10}", "Circumradius:", self.circumradius)?;
        writeln!(
            f,
            "│  {:>w$}  {:.10}",
            "Nine-point radius:", self.nine_point_radius
        )?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Medians")?;
        writeln!(f, "│  {:>w$}  {:.10}", "m_A:", self.median_a)?;
        writeln!(f, "│  {:>w$}  {:.10}", "m_B:", self.median_b)?;
        writeln!(f, "│  {:>w$}  {:.10}", "m_C:", self.median_c)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Altitudes")?;
        writeln!(f, "│  {:>w$}  {:.10}", "h_A:", self.altitude_a)?;
        writeln!(f, "│  {:>w$}  {:.10}", "h_B:", self.altitude_b)?;
        writeln!(f, "│  {:>w$}  {:.10}", "h_C:", self.altitude_c)?;

        writeln!(f, "│")?;
        writeln!(f, "├─ Angle Bisectors")?;
        writeln!(f, "│  {:>w$}  {:.10}", "t_A:", self.bisector_a)?;
        writeln!(f, "│  {:>w$}  {:.10}", "t_B:", self.bisector_b)?;
        writeln!(f, "│  {:>w$}  {:.10}", "t_C:", self.bisector_c)?;

        writeln!(f, "│")?;
        writeln!(f, "└─ Euler Line Verification")?;
        let og = self.centroid - self.circumcenter;
        let oh = self.orthocenter - self.circumcenter;
        let euler_cross = og.cross(oh).abs();
        if euler_cross < 1e-6 {
            writeln!(
                f,
                "│  Circumcenter, Centroid, and Orthocenter are collinear ✓"
            )?;
        } else {
            writeln!(
                f,
                "│  Error: Euler line residual is large: {euler_cross:.2e}"
            )?;
        }

        write!(f, "\n{sep}")
    }
}

pub const fn rad_to_deg(rad: f64) -> f64 {
    rad.to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq_pt(p1: Point, p2: Point) -> bool {
        approx_eq(p1.x, p2.x) && approx_eq(p1.y, p2.y)
    }

    #[test]
    fn test_classifications() {
        // Equilateral
        let side_eq = 1.0;
        assert_eq!(
            classify_sides(side_eq, side_eq, side_eq),
            SideClassification::Equilateral
        );

        // Right Isosceles
        let side_a = 1.0;
        let side_b = 1.0;
        let side_c = 2.0f64.sqrt();
        assert_eq!(
            classify_sides(side_a, side_b, side_c),
            SideClassification::Isosceles
        );

        let pi_half = std::f64::consts::PI / 2.0;
        let pi_quarter = std::f64::consts::PI / 4.0;
        assert_eq!(
            classify_angles(pi_half, pi_quarter, pi_quarter),
            AngleClassification::Right
        );

        // Obtuse Scalene
        assert_eq!(classify_sides(10.0, 6.0, 5.0), SideClassification::Scalene);
        // 10^2 = 100, 6^2 + 5^2 = 36 + 25 = 61. 100 > 61 => Obtuse
        let a = angle_from_sides(10.0, 6.0, 5.0);
        let b = angle_from_sides(6.0, 10.0, 5.0);
        let c = angle_from_sides(5.0, 10.0, 6.0);
        assert_eq!(classify_angles(a, b, c), AngleClassification::Obtuse);
    }

    #[test]
    fn test_compute_summary_right_3_4_5() {
        let tri = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(0.0, 3.0),
        );
        let s = compute_summary(&tri);

        assert!(approx_eq(s.side_a, 5.0));
        assert!(approx_eq(s.side_b, 3.0));
        assert!(approx_eq(s.side_c, 4.0));
        assert!(approx_eq(s.area, 6.0));
        assert!(approx_eq(s.perimeter, 12.0));
        assert_eq!(s.angle_class, AngleClassification::Right);
        assert_eq!(s.side_class, SideClassification::Scalene);

        // Centers
        // Centroid: (0+4+0)/3, (0+0+3)/3 = (4/3, 1)
        assert!(approx_eq_pt(s.centroid, Point::new(4.0 / 3.0, 1.0)));
        // Circumcenter: for right triangle is midpoint of hypotenuse = (2, 1.5)
        assert!(approx_eq_pt(s.circumcenter, Point::new(2.0, 1.5)));
        // Orthocenter: for right triangle is the vertex with right angle = (0, 0)
        assert!(approx_eq_pt(s.orthocenter, Point::new(0.0, 0.0)));
        // Incenter: (1, 1)
        assert!(approx_eq_pt(s.incenter, Point::new(1.0, 1.0)));
        assert!(approx_eq(s.inradius, 1.0));

        // Euler line verification (G is between O and H, OG:GH = 1:2)
        // O=(2, 1.5), G=(1.333, 1), H=(0, 0)
        // Vector OH = (-2, -1.5). Vector OG = (-0.666, -0.5).
        // OH = 3 * OG. Correct.
    }

    #[test]
    fn test_compute_summary_equilateral() {
        let h = 3.0f64.sqrt() / 2.0;
        let tri = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, h),
        );
        let s = compute_summary(&tri);

        assert_eq!(s.side_class, SideClassification::Equilateral);
        assert!(approx_eq(s.side_a, 1.0));
        assert!(approx_eq(s.side_b, 1.0));
        assert!(approx_eq(s.side_c, 1.0));

        // In equilateral, all centers coincide
        assert!(approx_eq_pt(s.centroid, s.incenter));
        assert!(approx_eq_pt(s.centroid, s.circumcenter));
        assert!(approx_eq_pt(s.centroid, s.orthocenter));
    }

    #[test]
    fn test_precision_issue_isosceles() {
        // A very thin isosceles triangle
        let tri = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(1000.0, 0.0),
            Point::new(500.0, 0.000_001),
        );

        // Area = 0.5 * 1000 * 0.000001 = 0.0005
        let s = compute_summary(&tri);
        assert!(s.area > 0.0);
        assert!(approx_eq(s.area, 0.0005));
        assert_eq!(s.side_class, SideClassification::Isosceles);
    }

    #[test]
    fn test_large_coordinates() {
        let tri = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(1e10, 0.0),
            Point::new(0.0, 1e10),
        );
        let s = compute_summary(&tri);
        assert!(s.area.is_finite());
        assert!(s.circumcenter.x.is_finite());
    }

    #[test]
    fn test_euler_line_and_nine_point_circle() {
        // Scalene Acute triangle
        let tri = Triangle::new(
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(3.0, 7.0),
        );
        let s = compute_summary(&tri);

        // Euler line: O, G, H are collinear
        let og = s.centroid - s.circumcenter;
        let oh = s.orthocenter - s.circumcenter;
        let euler_cross = og.cross(oh).abs();
        assert!(
            euler_cross < 1e-7,
            "Euler line cross product too large: {euler_cross}"
        );

        // G is between O and H, OH = 3 * OG
        assert!(approx_eq(
            s.circumcenter.distance_to(s.orthocenter),
            3.0 * s.circumcenter.distance_to(s.centroid)
        ));

        // Nine point center N is midpoint of OH
        let n_calc = (s.circumcenter + s.orthocenter) / 2.0;
        assert!(approx_eq_pt(s.nine_point_center, n_calc));

        // Nine point radius is half circumradius
        assert!(approx_eq(s.nine_point_radius, s.circumradius / 2.0));

        // Check if midpoint of side AB (5, 0) is on nine-point circle
        let midpoint_ab = (s.vertex_a + s.vertex_b) / 2.0;
        let dist_to_n = midpoint_ab.distance_to(s.nine_point_center);
        assert!(
            approx_eq(dist_to_n, s.nine_point_radius),
            "Midpoint not on nine-point circle: dist={dist_to_n}, r={}",
            s.nine_point_radius
        );
    }
}
