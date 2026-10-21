//! May be a copy of vec2

use std::ops;

// ------------------- VECX STRUCT/CLASS -------------------
// Where was this derived from? https://docs.rs/vec2/latest/vec2/ ???

#[derive(Clone, Copy, Debug)]
pub struct VecSpace {
    x: f64,
    y: f64,
}

impl ops::Add<VecSpace> for VecSpace {
    type Output = Self;

    fn add(self, other: VecSpace) -> VecSpace {
        VecSpace {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<VecSpace> for VecSpace {
    fn add_assign(&mut self, other: VecSpace) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<VecSpace> for VecSpace {
    type Output = Self;

    fn sub(self, other: VecSpace) -> VecSpace {
        VecSpace {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<f64> for VecSpace {
    type Output = Self;

    fn mul(self, scalar: f64) -> VecSpace {
        VecSpace::new(self.x * scalar, self.y * scalar)
    }
}

impl VecSpace {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn _set_x(&mut self, val: f64) {
        self.x = val;
    }

    pub fn add_x(&mut self, val: f64) {
        self.x += val;
    }

    pub fn _set_y(&mut self, val: f64) {
        self.y = val;
    }

    pub fn add_y(&mut self, val: f64) {
        self.y += val;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn _add(&self, other: VecSpace) -> VecSpace {
        VecSpace::new(self.x + other.x, self.y + other.y)
    }

    pub fn _sub(&self, other: VecSpace) -> VecSpace {
        VecSpace::new(self.x - other.x, self.y - other.y)
    }

    pub fn _mul(&self, scalar: f64) -> VecSpace {
        VecSpace::new(self.x * scalar, self.y * scalar)
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        // len >= epsilon
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    pub fn _normalized(&mut self) -> Self {
        let len = self.length();
        // len >= epsilon
        if len == 0.0 {
            return VecSpace::ZERO;
        }

        VecSpace {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    /// Returns the angle (in radians) of this vector in the range `[-π, +π]`.
    ///
    /// The input does not need to be a unit vector however it must be non-zero.
    #[inline]
    #[must_use]
    pub fn _to_angle(self) -> f64 {
        //core::f32::math::atan2(self.y, self.x)
        self.y.atan2(self.x)
    }

    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
    /// `Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
    /// and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(&self, angle: f64) -> Self {
        //let (sin, cos) = f32::math::sin_cos(angle);
        let sin = angle.sin();
        let cos = angle.cos();
        Self { x: cos, y: sin }
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[inline]
    #[must_use]
    pub fn rotate(&self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }

    pub fn rotate_angle(&self, angle: f64) -> Self {
        let x = self.from_angle(angle);
        self.rotate(x)
    }
}
