use crate::{
    points3d_iter::Points3DIter, points_iter::PointsIter, Error, Point, Point3D, Points, Points3D,
    Result, SplineOpts, SplineOpts3D,
};

///
/// The main function that does all the work.
///
/// Returns points of curve constructed within the range of passed points
/// using cubic spline interpolation.
///
/// # Example
/// ```
/// use cubic_spline::{Points, TryFrom, SplineOpts};
///
/// let src_points = vec![(1.0, 1.0), (3.3, 2.7), (5.1, 0.9)];
/// let prepared_points = Points::try_from(&src_points).expect("cant convert points");
///
/// let options = SplineOpts::new()
///   .tension(0.5)
///   .num_of_segments(16);
///
/// let calculated_points = prepared_points
///   .calc_spline(&options)
///   .expect("cant construct spline points");
///
/// assert_eq!(calculated_points.get_ref().len(), 33);
/// ```
pub fn calc_spline(points: &Points, opts: &SplineOpts) -> Result<Points> {
    let mut points_len = points.get_ref().len();

    if points_len < 2 {
        return Err(Error::TooFewPoints);
    }

    let tension_from_opt = opts.get_tension();
    let num_of_segments = opts.get_num_of_segments();

    let num_of_segments_f64 = f64::from(num_of_segments);

    // number of segments in the spaces between points
    // multiplied by the number of spaces
    // plus the last ending point, because the function calculates from a point to a point not inclusive
    if opts.get_closed() {
        points_len += 1; // add first point at the end
    }
    let generated_count = (points_len - 1) * (num_of_segments as usize) + 1;

    let mut result: Vec<Point> = Vec::with_capacity(generated_count);

    let iter = PointsIter::new(points, opts);

    for (prev, curr, next, next2) in iter {
        let tension = curr.tension.unwrap_or(tension_from_opt);

        let t1x = (next.x - prev.x) * tension;
        let t2x = (next2.x - curr.x) * tension;
        let t1y = (next.y - prev.y) * tension;
        let t2y = (next2.y - curr.y) * tension;

        for t in 0..num_of_segments {
            let st = f64::from(t) / num_of_segments_f64;
            let st_pow2 = st.powi(2);
            let st_pow3 = st.powi(3);
            let st_pow2x3 = 3.0 * st_pow2;
            let st_pow3x2 = 2.0 * st_pow3;

            let c1 = st_pow3x2 - st_pow2x3 + 1.0;
            let c2 = -st_pow3x2 + st_pow2x3;
            let c3 = st_pow3 - 2.0 * st_pow2 + st;
            let c4 = st_pow3 - st_pow2;

            let x = c1 * curr.x + c2 * next.x + c3 * t1x + c4 * t2x;
            let y = c1 * curr.y + c2 * next.y + c3 * t1y + c4 * t2y;

            result.push(Point::new(x, y));
        }
    }

    // unnecessary check. so as not to write unwrap
    if opts.get_closed() {
        if let Some(first) = points.get_ref().first() {
            result.push(Point::new(first.x, first.y));
        }
    } else if let Some(last) = points.get_ref().last() {
        // need to add the last one because the function calculates points
        // in the interval between point1 and point2 including the first, but not including the last one
        result.push(Point::new(last.x, last.y));
    }

    Ok(Points::from(result))
}

///
/// The main function that does all the work but for 3D.
///
/// Returns points of 3D curve constructed within the range of passed points
/// using cubic spline interpolation.
///
/// # Example
/// ```
/// use cubic_spline::{Points3D, TryFrom, SplineOpts};
///
/// let src_points = vec![(1.0, 1.0, 1.0), (3.3, 2.7, 1.5), (5.1, 0.9, 0.0)];
/// let prepared_points = Points3D::try_from(&src_points).expect("cant convert points");
///
/// let options = SplineOpts::new()
///   .tension(0.5)
///   .num_of_segments(16);
///
/// let calculated_points = prepared_points
///   .calc_spline(&options)
///   .expect("cant construct spline points");
///
/// assert_eq!(calculated_points.get_ref().len(), 33);
/// ```
pub fn calc_spline_3d(points: &Points3D, opts: &SplineOpts3D) -> Result<Points3D> {
    let mut points_len = points.get_ref().len();

    if points_len < 2 {
        return Err(Error::TooFewPoints);
    }

    let tension_from_opt = opts.get_tension();
    let num_of_segments = opts.get_num_of_segments();

    let num_of_segments_f64 = f64::from(num_of_segments);

    // number of segments in the spaces between points
    // multiplied by the number of spaces
    // plus the last ending point, because the function calculates from a point to a point not inclusive
    if opts.get_closed() {
        points_len += 1; // add first point at the end
    }
    let generated_count = (points_len - 1) * (num_of_segments as usize) + 1;

    let mut result: Vec<Point3D> = Vec::with_capacity(generated_count);

    let iter = Points3DIter::new(points, opts);

    for (prev, curr, next, next2) in iter {
        let tension = curr.tension.unwrap_or(tension_from_opt);

        let t1x = (next.x - prev.x) * tension;
        let t2x = (next2.x - curr.x) * tension;
        let t1y = (next.y - prev.y) * tension;
        let t2y = (next2.y - curr.y) * tension;
        let t1z = (next.z - prev.z) * tension;
        let t2z = (next2.z - curr.z) * tension;

        for t in 0..num_of_segments {
            let st = f64::from(t) / num_of_segments_f64;
            let st_pow2 = st.powi(2);
            let st_pow3 = st.powi(3);
            let st_pow2x3 = 3.0 * st_pow2;
            let st_pow3x2 = 2.0 * st_pow3;

            let c1 = st_pow3x2 - st_pow2x3 + 1.0;
            let c2 = -st_pow3x2 + st_pow2x3;
            let c3 = st_pow3 - 2.0 * st_pow2 + st;
            let c4 = st_pow3 - st_pow2;

            let x = c1 * curr.x + c2 * next.x + c3 * t1x + c4 * t2x;
            let y = c1 * curr.y + c2 * next.y + c3 * t1y + c4 * t2y;
            let z = c1 * curr.z + c2 * next.z + c3 * t1z + c4 * t2z;

            result.push(Point3D::new(x, y, z));
        }
    }

    // unnecessary check. so as not to write unwrap
    if opts.get_closed() {
        if let Some(first) = points.get_ref().first() {
            result.push(Point3D::new(first.x, first.y, first.z));
        }
    } else if let Some(last) = points.get_ref().last() {
        // need to add the last one because the function calculates points
        // in the interval between point1 and point2 including the first, but not including the last one
        result.push(Point3D::new(last.x, last.y, last.z));
    }

    Ok(Points3D::from(result))
}
