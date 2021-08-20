use euclid::default::Point2D;

pub trait Shape<const N: usize> {
    fn contains(&self, point: Point2D<f64>) -> bool;
    fn path(&self) -> [Point2D<f64>; N];
}

pub(crate) struct Triangle {
    pub(crate) a: Point2D<f64>,
    pub(crate) b: Point2D<f64>,
    pub(crate) c: Point2D<f64>,
}

impl Shape<3> for Triangle {
    // http://jsfiddle.net/PerroAZUL/zdaY8/1/ praise be
    fn contains(&self, point: Point2D<f64>) -> bool {
        let area = (-self.b.y * self.c.x
            + self.a.y * (-self.b.x + self.c.x)
            + self.a.x * (self.b.y - self.c.y)
            + self.b.x * self.c.y)
            / 2f64;
        let sign = if area < 0f64 { -1f64 } else { 1f64 };

        let s = (self.a.y * self.c.x - self.a.x * self.c.y
            + (self.c.y - self.a.y) * point.x
            + (self.a.x - self.c.x) * point.y)
            * sign;
        let t = (self.a.x * self.b.y - self.a.y * self.b.x
            + (self.a.y - self.b.y) * point.x
            + (self.b.x - self.a.x) * point.y)
            * sign;

        s.is_sign_positive() && t.is_sign_positive() && (s + t) < 2f64 * area * sign
    }

    fn path(&self) -> [Point2D<f64>; 3] {
        [self.a, self.b, self.c]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn within_triangle_test() {
        let point = Point2D::new(243., 166.);
        let a = Point2D::new(136., 7.);
        let b = Point2D::new(316., 177.);
        let c = Point2D::new(217., 419.);

        let triangle = Triangle { a, b, c };

        assert!(triangle.contains(point))
    }
}
