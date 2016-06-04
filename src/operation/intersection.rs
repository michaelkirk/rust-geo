use types::Point;

pub trait HasIntersection<T> {
    fn intersection(&self, rhs: &T) -> Option<Point>;
}

impl HasIntersection<Point> for Point {
    /// Returns the point if they are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::Point;
    /// use geo::operation::intersection::HasIntersection;
    ///
    /// let p1 = Point::new(1.0, 2.0);
    /// let p2 = Point::new(1.0, 2.0);
    /// assert_eq!(Some(p1), p1.intersection(&p2));
    ///
    /// let p1 = Point::new(1.0, 2.0);
    /// let p2 = Point::new(2.0, 2.0);
    /// assert_eq!(None, p1.intersection(&p2));
    /// ```
    fn intersection(&self, other_point: &Point) -> Option<Point> {
        if self.eq(other_point) {
            return Some(self.clone());
        } else {
            return None;
        }
    }
}
