use crate::{Point, Points, SplineOpts};

pub(crate) type PointsToCalc<'a> = (&'a Point, &'a Point, &'a Point, &'a Point);

pub(crate) struct PointsIter<'a> {
    max_index: usize,
    index: usize,
    points: &'a Points,
    current: [&'a Point; 4],
    hidden_point_at_end: Option<&'a Point>,
    closed: bool,
}

impl<'a> PointsIter<'a> {
    pub(crate) fn new(points: &'a Points, opts: &'a SplineOpts) -> Self {
        let pts = points.get_ref();
        let curr = &pts[0];
        let prev = if opts.get_closed() {
            &pts[pts.len() - 1]
        } else {
            opts.get_hidden_point_at_start().unwrap_or(curr)
        };
        let next = &pts[1];
        let next2 = if let Some(next2) = pts.get(2) {
            next2
        } else if opts.get_closed() {
            &pts[0]
        } else {
            next
        };

        let current = [prev, curr, next, next2];

        let max_index = if opts.get_closed() {
            pts.len()
        } else {
            pts.len() - 1
        };

        PointsIter {
            max_index,
            index: 0,
            points,
            current,
            hidden_point_at_end: opts.get_hidden_point_at_end(),
            closed: opts.get_closed(),
        }
    }

    fn get_points_to_calc(&self) -> PointsToCalc<'a> {
        let c = self.current;
        (&c[0], &c[1], &c[2], &c[3])
    }

    fn update_current(&mut self) {
        self.current[0] = self.current[1];
        self.current[1] = self.current[2];
        self.current[2] = self.current[3];

        let pts = self.points.get_ref();

        if self.closed {
            let last = if self.index == self.max_index - 2 {
                &pts[0]
            } else if self.index == self.max_index - 1 {
                &pts[1]
            } else {
                &pts[self.index + 2]
            };
            self.current[3] = last;
        } else {
            let last = if self.index == self.max_index - 1 {
                self.hidden_point_at_end.unwrap_or(self.current[2])
            } else {
                &pts[self.index + 2]
            };
            self.current[3] = last;
        }

        /*let last = if self.index == self.max_index - 1 {
            if self.closed {
                &pts[0]
            } else {
                self.hidden_point_at_end.unwrap_or(self.current[2])
            }
        } else {
            &pts[self.index + 2]
        };
        self.current[3] = last;*/
    }
}

impl<'a> Iterator for PointsIter<'a> {
    type Item = PointsToCalc<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.max_index {
            return None;
        }

        let points_to_calc = self.get_points_to_calc();
        self.index += 1;

        // This is because when calling `self.update_current()` the next state is calculated
        // Next, because the first state is calculated at the very beginning,
        // so as not to check for a zero index every time and not to store `hidden_point_at_start`
        if self.index != self.max_index {
            self.update_current();
        }

        Some(points_to_calc)
    }
}
