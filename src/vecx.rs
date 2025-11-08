use std::ops;

// ------------------- VECX STRUCT/CLASS -------------------
// use own file???
#[derive(Clone, Copy, Debug)]
pub struct VecX {
    pub x: f64,
    pub y: f64,
}

impl ops::Add<VecX> for VecX {
    type Output = Self;

    fn add(self, other: VecX) -> VecX {
        VecX {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<VecX> for VecX {
    fn add_assign(&mut self, other: VecX) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<VecX> for VecX {
    type Output = Self;

    fn sub(self, other: VecX) -> VecX {
        VecX {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<f64> for VecX {
    type Output = Self;

    fn mul(self, scalar: f64) -> VecX {
        VecX::new(self.x * scalar, self.y * scalar)
    }
}

impl VecX {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn _add(&self, other: VecX) -> VecX {
        VecX::new(self.x + other.x, self.y + other.y)
    }

    pub fn _sub(&self, other: VecX) -> VecX {
        VecX::new(self.x - other.x, self.y - other.y)
    }

    pub fn _mul(&self, scalar: f64) -> VecX {
        VecX::new(self.x * scalar, self.y * scalar)
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        // len >= epsilon
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    pub fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
