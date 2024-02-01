use crate::{Point3D, DEFAULT_SEGMENTS, DEFAULT_TENSION};

///
/// A list of options indicating how the spline should be calculated
///
/// ```
/// use cubic_spline::SplineOpts3D;
///
/// let o1 = SplineOpts3D::default();
///
/// let o2 = SplineOpts3D::new();
///
/// let o3 = SplineOpts3D::new()
///   .tension(0.5)
///   .num_of_segments(16);
///
/// ```
/// Options list:
/// * `tension` -
///   Sets the bending strength of the curve.
///   The usual value ranges from `0.0` (straight) to `1.0` (very rounded).
///   If not specified [`DEFAULT_TENSION`] will be used.
///
/// * `num_of_segments` -
///   The number of points to be calculated between each two known points.
///   If not specified [`DEFAULT_SEGMENTS`] will be used.
///
/// * `hidden_point_at_start` - A point that will not be drawn,
///   but the beginning of the graph will bend as if it is there.
///
/// * `hidden_point_at_end` - Same as previous, but affects the end of the graph.
///
/// * `closed` - If `true` the curve will be closed.
///
/// [`DEFAULT_TENSION`]: constant.DEFAULT_TENSION.html
/// [`DEFAULT_SEGMENTS`]: constant.DEFAULT_SEGMENTS.html
#[derive(Clone)]
pub struct SplineOpts3D {
    tension: f64,
    num_of_segments: u32,
    hidden_point_at_start: Option<Point3D>,
    hidden_point_at_end: Option<Point3D>,
    closed: bool,
}

impl SplineOpts3D {
    ///
    /// Creates new one with defaults.
    pub fn new() -> Self {
        SplineOpts3D::default()
    }

    ///
    /// Sets tension.
    pub fn tension(mut self, val: f64) -> Self {
        self.tension = val;
        self
    }

    ///
    /// Sets num_of_segments.
    pub fn num_of_segments(mut self, val: u32) -> Self {
        self.num_of_segments = val;
        self
    }

    ///
    /// Sets hidden_point_at_start.
    pub fn hidden_point_at_start<T: Into<Point3D>>(mut self, val: T) -> Self {
        self.hidden_point_at_start = Some(val.into());
        self
    }

    ///
    /// Sets hidden_point_at_end.
    pub fn hidden_point_at_end<T: Into<Point3D>>(mut self, val: T) -> Self {
        self.hidden_point_at_end = Some(val.into());
        self
    }

    ///
    /// Sets closed.
    pub fn closed(mut self, val: bool) -> Self {
        self.closed = val;
        self
    }

    //
    // Sets tension.
    pub fn get_tension(&self) -> f64 {
        self.tension
    }

    //
    // Sets num_of_segments.
    pub fn get_num_of_segments(&self) -> u32 {
        self.num_of_segments
    }

    //
    // Sets hidden_point_at_start.
    pub fn get_hidden_point_at_start(&self) -> Option<&Point3D> {
        self.hidden_point_at_start.as_ref()
    }

    //
    // Sets hidden_point_at_end.
    pub fn get_hidden_point_at_end(&self) -> Option<&Point3D> {
        self.hidden_point_at_end.as_ref()
    }

    //
    // Sets closed.
    pub fn get_closed(&self) -> bool {
        self.closed
    }
}

impl Default for SplineOpts3D {
    ///
    /// # Example
    /// ```
    /// use cubic_spline::{SplineOpts3D};
    /// let opts = SplineOpts3D::default();
    ///
    /// assert_eq!(opts.get_num_of_segments(), cubic_spline::DEFAULT_SEGMENTS);
    /// assert!((opts.get_tension() - cubic_spline::DEFAULT_TENSION).abs() < 1e-9);
    /// assert!(opts.get_hidden_point_at_end().is_none());
    ///
    /// ```
    fn default() -> Self {
        SplineOpts3D {
            tension: DEFAULT_TENSION,
            num_of_segments: DEFAULT_SEGMENTS,
            hidden_point_at_start: None,
            hidden_point_at_end: None,
            closed: false,
        }
    }
}
