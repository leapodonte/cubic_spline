use crate::calc_spline_3d;
use crate::{Error, Result, SplineOpts3D, TryFrom, DEFAULT_APPROX_EQ_PRECISION};

///
/// The point in 3d coordinate system.
///
#[derive(Clone, Default, Debug)]
pub struct Point3D {
    ///
    /// x-axis point value.
    pub x: f64,

    ///
    /// y-axis point value.
    pub y: f64,

    ///
    /// z-axis point value.
    pub z: f64,

    ///
    /// Optional tension of the curve between this point and the next point.
    pub tension: Option<f64>,
}

///
/// Wrapper for your source points.
/// Prepares and validates points before calculating spline.
/// Create it with [`try_from`]`/`[`try_into`].
/// Or if you are very confident in the validity of your points use usual `From/Into` traits,
/// which will return an empty `Vec` on error.
///
/// # Example
/// ```
/// use cubic_spline::{Points3D, TryFrom};
///
/// let src1 = vec![[1.2, 3.3, 1.0], [122.2, 333.3, 100.0]];
/// let src2 = [[1.2, 3.3, 1.0], [122.2, 333.3, 100.0]];
/// let src3 = [(1.2, 3.3, 1.0), (122.2, 333.3, 100.0)];
/// let src4 = [1.2, 3.3, 1.0, 122.2, 333.3, 100.0];
///
/// assert!(Points3D::try_from(&src1).is_ok());
/// assert!(Points3D::try_from(&src2).is_ok());
/// assert_eq!(Points3D::from(&src3).get_ref().len(), 2);
///
/// let points1 = Points3D::try_from_flatten(&src4).unwrap();
/// let points2: Points3D = src1.into();
/// let first1 = points1.get_ref().first().unwrap();
///
/// assert!(first1.approx_eq(&points2.into_inner()[0]));
///
/// ```
///
/// [`try_from`]: trait.TryFrom.html#tymethod.try_from
/// [`try_into`]: trait.TryInto.html#tymethod.try_into
///
#[derive(Clone, Debug)]
pub struct Points3D(Vec<Point3D>);

//
//
//
//
//////////////////////////////////////////////////////
// POINT3D OWN IMPL
//////////////////////////////////////////////////////
impl Point3D {
    ///
    /// Creates new point. You may prefer use `From`/`Into` implementations for this.
    ///
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x,
            y,
            z,
            tension: None,
        }
    }

    ///
    /// Creates new point with tension of the curve between it and the next point.
    /// If points creates with `::new` the tension from [`SplineOpts3D`] will be used.
    ///
    /// [`SplineOpts3D`]: struct.SplineOpts3D.html
    pub fn with_tension(x: f64, y: f64, z: f64, tension: f64) -> Self {
        Point3D {
            x,
            y,
            z,
            tension: Some(tension),
        }
    }

    ///
    /// Tests the approximate equality of two points with default precision -
    /// [`DEFAULT_APPROX_EQ_PRECISION`]
    /// ```
    /// use cubic_spline::Point3D;
    ///
    /// assert!(Point3D::new(1.2,3.5,1.0).approx_eq(&[1.2, 3.5, 1.0].into()));
    ///
    /// ```
    ///
    /// [`DEFAULT_APPROX_EQ_PRECISION`]: static.DEFAULT_APPROX_EQ_PRECISION.html
    pub fn approx_eq(&self, other: &Point3D) -> bool {
        ((self.x - other.x).abs() < DEFAULT_APPROX_EQ_PRECISION)
            && ((self.y - other.y).abs() < DEFAULT_APPROX_EQ_PRECISION)
            && ((self.z - other.z).abs() < DEFAULT_APPROX_EQ_PRECISION)
    }

    ///
    /// Tests the approximate equality with specific precision
    ///
    /// ```
    /// use cubic_spline::Point;
    ///
    /// assert!(Point::new(1.0,1.0).approx_eq_with_precision(&[1.000_001, 1.0].into(), 0.000_1));
    /// assert!(Point::new(1.000_1,1.0).approx_eq_with_precision(&[1.0, 1.0].into(), 0.01));
    /// ```
    ///
    pub fn approx_eq_with_precision(&self, other: &Point3D, precision: f64) -> bool {
        ((self.x - other.x).abs() < precision)
            && ((self.y - other.y).abs() < precision)
            && ((self.z - other.z).abs() < precision)
    }

    ///
    /// Inverts the x-value of the point based on the width of the canvas.
    ///
    /// # Example
    /// ```
    /// use cubic_spline::Point;
    ///
    /// let mut p = Point::new(1.0, 3.0);
    /// p.invert_horizontally(3.0);
    ///
    /// assert_eq!(p.x, 2.0);
    /// assert_eq!(p.y, 3.0);
    ///
    /// ```
    pub fn invert_horizontally(&mut self, width: f64) {
        self.x = width - self.x;
    }

    ///
    /// Inverts the y-value of the point based on the height of the canvas.
    ///
    /// # Example
    /// ```
    /// use cubic_spline::Point;
    ///
    /// let mut p = Point::new(1.0, 3.0);
    /// p.invert_vertically(7.0);
    ///
    /// assert_eq!(p.x, 1.0);
    /// assert_eq!(p.y, 4.0);
    ///
    /// ```
    pub fn invert_vertically(&mut self, height: f64) {
        self.y = height - self.y;
    }
}

//
//
//
//
//////////////////////////////////////////////////////
// POINT FROM IMPL
//////////////////////////////////////////////////////

impl From<(f64, f64, f64)> for Point3D {
    fn from(p: (f64, f64, f64)) -> Self {
        Point3D::new(p.0, p.1, p.2)
    }
}

impl<'a> From<&'a Point3D> for Point3D {
    fn from(p: &'a Point3D) -> Self {
        p.clone()
    }
}

impl<'a, T: Copy> From<&'a T> for Point3D
where
    T: Into<Point3D>,
{
    fn from(p: &'a T) -> Self {
        (*p).into()
    }
}

impl<T> From<[T; 3]> for Point3D
where
    (T, T, T): Into<Point3D>,
{
    fn from([x, y, z]: [T; 3]) -> Self {
        (x, y, z).into()
    }
}

//
//
//
//
//////////////////////////////////////////////////////
// LIST OF POINTS3D OWN IMPL
//////////////////////////////////////////////////////
impl Points3D {
    ///
    /// Gets a reference to the underlying `Vec<Point3D>`.
    ///
    pub fn get_ref(&self) -> &Vec<Point3D> {
        &self.0
    }

    ///
    /// Gets a mutable reference to the underlying `Vec<Point3D>`.
    ///
    pub fn get_mut(&mut self) -> &mut Vec<Point3D> {
        &mut self.0
    }

    ///
    /// Consumes the `Points`, returning the wrapped `Vec<Point3D>`.
    ///
    pub fn into_inner(self) -> Vec<Point3D> {
        self.0
    }

    ///
    /// Similar to [`try_from`] but takes a flatten sequence of `f64` numbers
    /// where value at even index is `x` and value at odd index is `y`
    /// (e.g. `vec![12.0f64, 12.77, 15.3, 17.9]`, `[x,y,x,y,x...]`).
    ///
    /// [`try_from`]: trait.TryFrom.html#tymethod.try_from
    pub fn try_from_flatten<'a, I: IntoIterator<Item = &'a f64>>(into_f64_iter: I) -> Result<Self> {
        let mut v = Vec::new();

        let mut x = None;
        let mut y = None;

        for point in into_f64_iter.into_iter() {
            match (x, y) {
                (Some(px), Some(py)) => {
                    v.push(Point3D::new(px, py, *point)); // Assuming Point3D::new(x, y, z) exists
                    x = None;
                    y = None;
                }
                (Some(_px), None) => {
                    y = Some(*point);
                }
                (None, _) => {
                    x = Some(*point);
                }
            }
        }

        if x.is_some() || y.is_some() {
            return Err(Error::MissingCoordinate); // You may need to define this error variant
        }
        if v.len() < 2 {
            return Err(Error::TooFewPoints);
        }

        Ok(Points3D(v))
    }

    ///
    /// Inverts the x-value of all points based on the width of the canvas.
    ///
    /// # Example
    /// ```
    /// use cubic_spline::Points;
    ///
    /// let mut pts = Points::from(&[(1.0, 3.0), (2.0, 2.0)]);
    /// pts.invert_horizontally(7.0);
    ///
    /// let inverted: Vec<(f64,f64)> = pts.into();
    /// assert_eq!(&[(6.0, 3.0), (5.0,2.0)].as_ref(), &inverted );
    ///
    /// ```
    /*pub fn invert_horizontally(&mut self, width: f64) {
        self.0.iter_mut().for_each(|p| p.invert_horizontally(width));
    }*/

    ///
    /// Inverts the y-value of all points based on the height of the canvas.
    ///
    /// # Example
    /// ```
    /// use cubic_spline::Points;
    ///
    /// let mut pts = Points::from(&[(1.0, 3.0), (2.0, 2.0)]);
    /// pts.invert_vertically(7.0);
    ///
    /// let inverted: Vec<(f64,f64)> = pts.into();
    /// assert_eq!(&[(1.0, 4.0), (2.0,5.0)].as_ref(), &inverted );
    ///
    /// ```
    /*pub fn invert_vertically(&mut self, height: f64) {
        self.0.iter_mut().for_each(|p| p.invert_vertically(height));
    }*/

    ///
    /// The main function that does all the work.
    ///
    /// Returns points of curve constructed within the range of passed points
    /// using cubic spline interpolation.
    ///
    /// # Example
    /// ```
    /// use cubic_spline::{Points, TryFrom, SplineOpts3D};
    ///
    /// let src_points = vec![(1.0, 1.0), (3.3, 2.7), (5.1, 0.9)];
    /// let prepared_points = Points::try_from(&src_points).expect("cant convert points");
    ///
    /// let options = SplineOpts3D::new()
    ///   .tension(0.7)
    ///   .num_of_segments(16);
    ///
    /// let calculated_points = prepared_points.calc_spline(&options).unwrap();
    ///
    /// assert_eq!(calculated_points.get_ref().len(), 33);
    /// ```
    pub fn calc_spline(&self, opts: &SplineOpts3D) -> Result<Points3D> {
        calc_spline_3d(self, opts)
    }
}

//
//
//
//
//////////////////////////////////////////////////////
// LIST OF POINTS FROM IMPL
//////////////////////////////////////////////////////

impl<I: IntoIterator> From<I> for Points3D
where
    I::Item: Into<Point3D>,
{
    fn from(points: I) -> Self {
        Points3D(points.into_iter().map(Into::into).collect())
    }
}

impl<I: IntoIterator> TryFrom<I> for Points3D
where
    I::Item: Into<Point3D>,
{
    type Error = Error;
    fn try_from(points: I) -> Result<Self> {
        let v: Vec<Point3D> = points.into_iter().map(Into::into).collect();
        if v.len() < 3 {
            return Err(Error::TooFewPoints);
        }
        Ok(Points3D(v))
    }
}

impl From<Points3D> for Vec<(f64, f64, f64)> {
    fn from(pts: Points3D) -> Self {
        pts.get_ref().iter().map(|p| (p.x, p.y, p.z)).collect()
    }
}

impl From<Points3D> for Vec<[f64; 3]> {
    fn from(pts: Points3D) -> Self {
        pts.get_ref().iter().map(|p| [p.x, p.y, p.z]).collect()
    }
}

impl From<Points3D> for Vec<f64> {
    fn from(pts: Points3D) -> Self {
        let mut res = Vec::with_capacity(pts.0.len() * 3);
        pts.get_ref().iter().for_each(|p| {
            res.push(p.x);
            res.push(p.y);
            res.push(p.z);
        });
        res
    }
}

//
//
//
//
//////////////////////////////////////////////////////
// TESTS
//////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use crate::{Error, Points3D, TryFrom};

    fn points_eq(pp1: &Points3D, pp2: &Points3D) -> bool {
        pp1.get_ref()
            .iter()
            .zip(pp2.get_ref().iter())
            .all(|(p1, p2)| p1.approx_eq_with_precision(p2, 0.000_1))
    }

    #[test]
    fn from() {
        let src1 = vec![[1.2, 3.3, 1.0], [122.2, 333.3, 100.0]];
        let src2 = [[1.2, 3.3, 1.0], [122.2, 333.3, 100.0]];
        let src3 = [(1.2, 3.3, 1.0), (122.2, 333.3, 100.0)];
        let src4 = [1.2, 3.3, 1.0, 122.2, 333.3, 100.0];

        assert!(points_eq(&Points3D::from(&src1), &Points3D::from(&src2)));
        assert!(points_eq(&Points3D::from(&src1), &Points3D::from(&src3)));
        assert!(points_eq(
            &Points3D::from(&src1),
            &Points3D::try_from_flatten(&src4).unwrap(),
        ));

        let src5 = vec![[1.2, 3.3, 1.0]];
        let src6 = [1.2, 3.3, 1.0, 122.2];

        assert_eq!(Points3D::try_from(&src5).unwrap_err(), Error::TooFewPoints);
        assert_eq!(
            Points3D::try_from_flatten(&src6).unwrap_err(),
            Error::MissingY
        );
    }
}
