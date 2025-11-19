use std::ops;

// ------------------- VECX STRUCT/CLASS -------------------
// use own file???
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

    pub fn normalized(&mut self) -> Self {
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
}
