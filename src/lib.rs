#![deny(missing_docs)]

//! Helper methods for computing simple rectangle layout.

extern crate float;

use float::Float;

/// Helper methods for computing simple rectangle layout.
pub trait Rect: Sized {
    /// The internal scalar type.
    type Scalar: Float;
    /// Creates new rectangle from x, y, w, h.
    fn from_x_y_w_h(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)
        -> Self;
    /// Converts from u32 rectangle.
    #[inline(always)]
    fn from_u32(rect: [u32; 4]) -> Self {
        use float::FromPrimitive;

        Rect::from_x_y_w_h(
            FromPrimitive::from_u32(rect[0]),
            FromPrimitive::from_u32(rect[1]),
            FromPrimitive::from_u32(rect[2]),
            FromPrimitive::from_u32(rect[3])
        )
    }
    /// Converts from i32 rectangle.
    #[inline(always)]
    fn from_i32(rect: [i32; 4]) -> Self {
        use float::FromPrimitive;

        Rect::from_x_y_w_h(
            FromPrimitive::from_i32(rect[0]),
            FromPrimitive::from_i32(rect[1]),
            FromPrimitive::from_i32(rect[2]),
            FromPrimitive::from_i32(rect[3])
        )
    }
    /// Gets x.
    fn x(&self) -> Self::Scalar;
    /// Gets y.
    fn y(&self) -> Self::Scalar;
    /// Gets w.
    fn w(&self) -> Self::Scalar;
    /// Gets h.
    fn h(&self) -> Self::Scalar;
    /// Sets x.
    fn set_x(&mut self, val: Self::Scalar);
    /// Sets y.
    fn set_y(&mut self, val: Self::Scalar);
    /// Sets w.
    fn set_w(&mut self, val: Self::Scalar);
    /// Sets h.
    fn set_h(&mut self, val: Self::Scalar);
    /// Returns x and y.
    #[inline(always)]
    fn xy(&self) -> (Self::Scalar, Self::Scalar) { (self.x(), self.y()) }
    /// Returns w and h.
    #[inline(always)]
    fn wh(&self) -> (Self::Scalar, Self::Scalar) { (self.w(), self.h()) }
    /// Return x and w.
    #[inline(always)]
    fn xw(&self) -> (Self::Scalar, Self::Scalar) { (self.x(), self.w()) }
    /// Returns y and h.
    #[inline(always)]
    fn yh(&self) -> (Self::Scalar, Self::Scalar) { (self.y(), self.h()) }
    /// Returns left and right.
    #[inline(always)]
    fn x1x2(&self) -> (Self::Scalar, Self::Scalar) { (self.x(), self.x() + self.w()) }
    /// Returns top and bottom.
    #[inline(always)]
    fn y1y2(&self) -> (Self::Scalar, Self::Scalar) { (self.y(), self.y() + self.h()) }
    /// Returns upper left and lower right corner.
    #[inline(always)]
    fn p1p2(&self) -> ([Self::Scalar; 2], [Self::Scalar; 2]) {
        ([self.x(), self.y()], [self.x() + self.w(), self.y() + self.h()])
    }
    /// Returns x, y, w, h.
    #[inline(always)]
    fn xywh(&self) -> (Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar) {
        (self.x(), self.y(), self.w(), self.h())
    }
    /// Returns the center of the rectangle.
    #[inline(always)]
    fn center(&self) -> [Self::Scalar; 2] {
        use float::FromPrimitive;

        let _05: Self::Scalar = FromPrimitive::from_f64(0.5);
        [self.x() + _05 * self.h(), self.y() + _05 * self.h()]
    }
    /// Returns true if the rectangle is empty.
    #[inline(always)]
    fn is_empty(&self) -> bool {
        use float::Zero;

        self.w() * self.h() == Zero::zero()
    }
    /// Computes a margin rectangle.
    /// If the margin is too large, an empty rectangle in the middle is returned.
    fn margin(&self, val: Self::Scalar) -> Self {
        use float::{ FromPrimitive, Zero };

        let x: Self::Scalar;
        let y: Self::Scalar;
        let w: Self::Scalar;
        let h: Self::Scalar;
        let _2: Self::Scalar = FromPrimitive::from_f64(2.0);
        let _05: Self::Scalar = FromPrimitive::from_f64(0.5);
        let _0: Self::Scalar = Zero::zero();
        if self.w() < _2 * val {
            x = self.x() + _05 * self.w();
            w = _0;
        } else {
            x = self.x() + val;
            w = self.w() - _2 * val;
        }
        if self.h() < _2 * val {
            y = self.y() + _05 * self.h();
            h = _0;
        } else {
            y = self.y() + val;
            h = self.h() - _2 * val;
        }
        <Self as Rect>::from_x_y_w_h(x, y, w, h)
    }
    /// Splits from the left side of rectangle up to a factor.
    fn split_left(&self, val: Self::Scalar, factor: Self::Scalar)
    -> (Self, Self) {
        use float::One;

        let (x, y, w, h) = self.xywh();
        if val > w * factor {
            let _1: Self::Scalar = One::one();
            (Rect::from_x_y_w_h(x, y, w * factor, h),
             Rect::from_x_y_w_h(x + w * factor, y, w * (_1 - factor), h))
        } else {
            (Rect::from_x_y_w_h(x, y, val, h),
             Rect::from_x_y_w_h(x + val, y, w - val, h))
         }
    }
    /// Splits from the right side of rectangle.
    #[inline(always)]
    fn split_right(&self, val: Self::Scalar, factor: Self::Scalar)
    -> (Self, Self) {
        use float::One;

        let (x, y, w, h) = self.xywh();
        if val > w * factor {
            let _1: Self::Scalar = One::one();
            (Rect::from_x_y_w_h(x, y, w * (_1 - factor), h),
             Rect::from_x_y_w_h(x + w * (_1 - factor), y, w * factor, h))
        } else {
            (Rect::from_x_y_w_h(x, y, w - val, h),
             Rect::from_x_y_w_h(x + w - val, y, val, h))
         }
    }
    /// Splits from the top side of rectangle.
    fn split_top(&self, val: Self::Scalar, factor: Self::Scalar)
    -> (Self, Self) {
        use float::One;

        let (x, y, w, h) = self.xywh();
        if val > h * factor {
            let _1: Self::Scalar = One::one();
            (Rect::from_x_y_w_h(x, y, w, h * factor),
             Rect::from_x_y_w_h(x, y + h * factor, w, h * (_1 - factor)))
        } else {
            (Rect::from_x_y_w_h(x, y, w, val),
             Rect::from_x_y_w_h(x, y + val, w, h - val))
         }
    }
    /// Splits from the bottom side of the rectangle.
    #[inline(always)]
    fn split_bottom(&self, val: Self::Scalar, factor: Self::Scalar)
    -> (Self, Self) {
        use float::One;

        let (x, y, w, h) = self.xywh();
        if val > h * factor {
            let _1: Self::Scalar = One::one();
            (Rect::from_x_y_w_h(x, y, w, h * (_1 - factor)),
             Rect::from_x_y_w_h(x, y + h * (_1 - factor), w, h * factor))
        } else {
            (Rect::from_x_y_w_h(x, y, w, h - val),
             Rect::from_x_y_w_h(x, y + h - val, w, val))
         }
    }
}

impl Rect for [f64; 4] {
    type Scalar = f64;
    fn from_x_y_w_h(x: f64, y: f64, w: f64, h: f64) -> [f64; 4] {
        [x, y, w, h]
    }
    fn x(&self) -> f64 { self[0] }
    fn y(&self) -> f64 { self[1] }
    fn w(&self) -> f64 { self[2] }
    fn h(&self) -> f64 { self[3] }
    fn set_x(&mut self, val: f64) { self[0] = val }
    fn set_y(&mut self, val: f64) { self[1] = val }
    fn set_w(&mut self, val: f64) { self[2] = val }
    fn set_h(&mut self, val: f64) { self[3] = val }
}


impl Rect for [f32; 4] {
    type Scalar = f32;
    fn from_x_y_w_h(x: f32, y: f32, w: f32, h: f32) -> [f32; 4] {
        [x, y, w, h]
    }
    fn x(&self) -> f32 { self[0] }
    fn y(&self) -> f32 { self[1] }
    fn w(&self) -> f32 { self[2] }
    fn h(&self) -> f32 { self[3] }
    fn set_x(&mut self, val: f32) { self[0] = val }
    fn set_y(&mut self, val: f32) { self[1] = val }
    fn set_w(&mut self, val: f32) { self[2] = val }
    fn set_h(&mut self, val: f32) { self[3] = val }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        assert!([0.0, 0.0, 0.0, 0.0].is_empty());
        assert!([0.0, 0.0, 1.0, 0.0].is_empty());
        assert!([0.0, 0.0, 0.0, 1.0].is_empty());
        assert!(![0.0, 0.0, 1.0, 1.0].is_empty());
    }

    #[test]
    fn margin() {
        assert_eq!([0.0, 0.0, 0.0, 0.0].margin(2.0), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!([0.0, 0.0, 1.0, 1.0].margin(2.0), [0.5, 0.5, 0.0, 0.0]);
        assert_eq!([0.0, 0.0, 100.0, 100.0].margin(2.0), [2.0, 2.0, 96.0, 96.0]);
        assert_eq!([0.0, 0.0, 100.0, 200.0].margin(2.0), [2.0, 2.0, 96.0, 196.0]);
    }

    #[test]
    fn xywh() {
        let rect = [0.0, 1.0, 2.0, 3.0];
        assert_eq!(rect.x(), 0.0);
        assert_eq!(rect.y(), 1.0);
        assert_eq!(rect.w(), 2.0);
        assert_eq!(rect.h(), 3.0);
    }

    #[test]
    fn split_left() {
        assert_eq!([0.0, 0.0, 0.0, 10.0].split_left(30.0, 0.5),
            ([0.0, 0.0, 0.0, 10.0], [0.0, 0.0, 0.0, 10.0]));
        assert_eq!([0.0, 0.0, 10.0, 10.0].split_left(15.0, 0.5),
            ([0.0, 0.0, 5.0, 10.0], [5.0, 0.0, 5.0, 10.0]));
        assert_eq!([0.0, 0.0, 100.0, 10.0].split_left(30.0, 0.5),
            ([0.0, 0.0, 30.0, 10.0], [30.0, 0.0, 70.0, 10.0]));
    }

    #[test]
    fn split_top() {
        assert_eq!([0.0, 0.0, 10.0, 0.0].split_top(30.0, 0.5),
            ([0.0, 0.0, 10.0, 0.0], [0.0, 0.0, 10.0, 0.0]));
        assert_eq!([0.0, 0.0, 10.0, 10.0].split_top(15.0, 0.5),
            ([0.0, 0.0, 10.0, 5.0], [0.0, 5.0, 10.0, 5.0]));
        assert_eq!([0.0, 0.0, 10.0, 100.0].split_top(30.0, 0.5),
            ([0.0, 0.0, 10.0, 30.0], [0.0, 30.0, 10.0, 70.0]));
    }
}
