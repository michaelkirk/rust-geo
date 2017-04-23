use types::{Geometry, Point, LineString};
use num_traits::Float;

pub trait Intersection<T, G> {
    fn intersection(&self, rhs: &G) -> Option<Geometry<T>> where T: Float;
}

impl<T> Intersection<T, Point<T>> for Point<T>
    where T: Float {
    /// Returns the point if they are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{Geometry, Point};
    /// use geo::operation::intersection::Intersection;
    ///
    /// let p1 = Point::new(1.0, 2.0);
    /// let p2 = Point::new(1.0, 2.0);
    /// assert_eq!(Some(Geometry::Point(p1)), p1.intersection(&p2));
    ///
    /// let p1 = Point::new(1.0, 2.0);
    /// let p2 = Point::new(2.0, 2.0);
    /// assert_eq!(None, p1.intersection(&p2));
    /// ```
    fn intersection(&self, other_point: &Point<T>) -> Option<Geometry<T>> {

        if self.eq(other_point) {
            return Some(Geometry::Point(self.clone()));
        } else {
            return None;
        }
    }
}

impl<T> Intersection<T, Point<T>> for LineString<T>
    where T: Float {
    /// Returns the point if it's on the LineString.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{Geometry, Point, LineString};
    /// use geo::operation::intersection::Intersection;
    ///
    /// let line_start = Point::new(1.0, 1.0);
    /// let in_line = Point::new(2.0, 2.0);
    /// let line_string = LineString(vec![
    ///   Point::new(1.0, 1.0),
    ///   Point::new(3.0, 3.0)
    /// ]);
    ///
    /// assert_eq!(Some(Geometry::Point(line_start)), line_string.intersection(&line_start));
    /// assert_eq!(Some(Geometry::Point(in_line)), line_string.intersection(&in_line));
    ///
    /// let off_line = Point::new(1.0, 2.0);
    /// assert_eq!(None, line_string.intersection(&off_line));
    ///
    /// let past_line = Point::new(4.0, 4.0);
    /// assert_eq!(None, line_string.intersection(&past_line));
    ///
    /// let past_line = Point::new(1.0, 4.0);
    /// let vertical_line_string = LineString(vec![
    ///   Point::new(1.0, 1.0),
    ///   Point::new(1.0, 3.0)
    /// ]);
    /// assert_eq!(None, vertical_line_string.intersection(&past_line));
    /// ```
    fn intersection(&self, point: &Point<T>) -> Option<Geometry<T>> {
        for (start, end) in self.0.iter().zip(self.0[1..].iter()) {
            let dx_point = point.x() - start.x();
            let dy_point = point.y() - start.y();
            let dx_line = end.x() - start.x();
            let dy_line = end.y() - start.y();

            let cross_product_magnitude = dx_point * dy_line - dy_point * dx_line;

            if cross_product_magnitude != T::zero() {
                continue;
            }

            // point is on the line extending from the segment, but is it within the segment?

            let coord = if dx_line == T::zero() {
                // All points on a vertical line have the same x, so we must compare y values
                Point::y
            } else {
                Point::x
            };

            let (lower_bound, upper_bound) = if coord(start) < coord(end) {
                (coord(start), coord(end))
            } else {
                (coord(end), coord(start))
            };

            if lower_bound <= coord(point) && coord(point) <= upper_bound {
                return Some(Geometry::Point(point.clone()));
            }
        }
        return None;
    }
}

impl<T> Intersection<T, LineString<T>> for LineString<T>
    where T: Float {
    /// Returns any overlapping line segements and intersecting points
    ///
    /// # Examples
    ///
    /// ```
    /// use geo::{Geometry, Point, LineString};
    /// use geo::operation::intersection::Intersection;
    ///
    /// let line_string = LineString(vec![
    ///   Point::new(1.0, 1.0),
    ///   Point::new(3.0, 3.0)
    /// ]);
    ///
    /// let same_line_string = LineString(vec![
    ///   Point::new(1.0, 1.0),
    ///   Point::new(3.0, 3.0)
    /// ]);
    ///
    /// let far_away_line_string = LineString(vec![
    ///   Point::new(4.0, 4.0),
    ///   Point::new(5.0, 5.0)
    /// ]);
    ///
    /// assert_eq!(Some(Geometry::LineString(line_string.clone())), line_string.intersection(&same_line_string));
    /// assert_eq!(None, line_string.intersection(&far_away_line_string));
    /// ```
    fn intersection(&self, other_line_string: &LineString<T>) -> Option<Geometry<T>> {
        // TODO actually implement this method
        if self.0.eq(&other_line_string.0) {
            return Some(Geometry::LineString(self.clone()));
        } else {
            return None
        }
    }
}
