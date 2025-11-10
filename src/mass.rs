use crate::vecx::*;
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 680; // ??? calculate frame
const WINDOW_CENTER: VecX = VecX {
    x: (WINDOW_WIDTH / 2) as f64,
    y: (WINDOW_HEIGHT / 2) as f64,
};

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)
pub const M_AE: f64 = 149_597_870_700.0; // m per Astronomic Unit
pub const MAX_GRAVITY_DISTANCE: f64 = 1e38; // [AE]
pub const DRAW_FACT: f64 = 200.0;
pub const DRAW_MIN: i32 = 3;
pub const DRAW_MAX: i32 = 200;

// Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
// The smallest extend of the window counts as visible screen range
const PIXEL: i32 = WINDOW_HEIGHT / 2; // todo: do it dynamic!

// ------------------- MASS STRUCT/CLASS -------------------

#[derive(Debug, Default)]
pub struct MassData<'a> {
    pub name: &'a str,
    pub color: Color,
    pub diameter: f64,
    pub mass: f64,
    pub radius: f64,
    pub excentricity: f64,
}

#[derive(Debug, Clone)]
pub struct Mass {
    position: VecX,
    velocity: VecX,
    saved_position: VecX,
    saved_velocity: VecX,
    acceleration: VecX,
    color: Color,
    name: String,
    mass: f64,
    diameter: f64,
    pub prediction: Vec<VecX>,
}

impl Mass {
    // "Static" constants

    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        let position = VecX::new(0.0, data.radius);
        let velocity = VecX::ZERO;
        let acceleration = VecX::ZERO;

        let mut mass = Mass {
            name: data.name.to_string(),
            color: data.color,
            diameter: data.diameter,
            mass: data.mass,
            saved_position: VecX::ZERO,
            saved_velocity: VecX::ZERO,
            position,
            velocity,
            acceleration,
            prediction: Vec::new(),
        };

        if orbits.is_some() {
            Self::mass_v_orbit(&mut mass, &mut orbits.unwrap(), data.excentricity);
        }

        return mass;
    }

    /// Computes orbital velocity for a circular orbit
    /// around a body with `central_mass` at distance `radius` (in meters)

    fn mass_v_orbit(mass: &mut Mass, other: &mut Mass, excentriticy: f64) {
        let signum = if mass.position.y > 0.0 { 1.0 } else { -1.0 };
        mass.position += other.position;
        mass.velocity += other.velocity;
        let radius = (other.position - mass.position).length();

        let both_masses = mass.mass + other.mass;
        let velocity =
            (GRAVITY_CONSTANT_OF_EARTH * both_masses / radius).sqrt() * (1. - excentriticy);
        mass.velocity.x += velocity / both_masses * other.mass * signum;
        other.velocity.x += -velocity / both_masses * mass.mass * signum;
    }

    fn _v_orbit(central_mass: f64, radius: f64) -> f64 {
        (GRAVITY_CONSTANT_OF_EARTH * central_mass / radius).sqrt()
    }

    pub fn save(&mut self) {
        self.saved_position = self.position;
        self.saved_velocity = self.velocity;
    }

    pub fn restore(&mut self) {
        self.position = self.saved_position;
        self.velocity = self.saved_velocity;
    }

    pub fn dragged_by(&mut self, other: &Mass) {
        if (self as *const _) == (other as *const _) || other.mass == 0.0 {
            return; // don’t drag self or zero-mass objects
        }

        let mut distance_vector = other.position - self.position;
        let distance = distance_vector.length();

        if distance < MAX_GRAVITY_DISTANCE * M_AE {
            distance_vector.normalize();

            let acceleration = other.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
            let acceleration_vector = distance_vector * acceleration;

            self.acceleration += acceleration_vector;
        }
    }

    pub fn frame_move(&mut self, dt: f64) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration.set_zero();
    }

    pub fn draw(&self) {
        // sqrt(sqrt()) scaling like Kotlin code
        let mut size = ((self.diameter / M_AE).sqrt().sqrt() / 2.0 * DRAW_FACT) as i32;
        size = size.clamp(DRAW_MIN, DRAW_MAX);

        let screen_pos = scale(self.position);
        draw_circle(
            screen_pos.x as f32,
            screen_pos.y as f32,
            size as f32,
            self.color,
        );

        let mut last = scale(self.position);
        for position in &self.prediction {
            let this = scale(position.clone());

            if false {
                draw_line(
                    last.x as f32,
                    last.y as f32,
                    this.x as f32,
                    this.y as f32,
                    0.1,
                    WHITE,
                );
            } else {
                draw_rectangle(this.x as f32, this.y as f32, 1., 1., WHITE);
            }

            last = this;
        }
    }
}

fn scale(position: VecX) -> VecX {
    let z_view = 1.2;
    position * (PIXEL as f64 / z_view / M_AE) + WINDOW_CENTER
}

// ------------------- MASSES STRUCT/CLASS -------------------
pub struct Masses {
    masses: Vec<Mass>,
}

impl Masses {
    pub fn new() -> Masses {
        Masses { masses: Vec::new() }
    }

    pub fn add_at_place(&mut self, data: &MassData) -> usize {
        let mass = Mass::new(data, None);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        let orbits = &mut self.masses[orbits];
        let mass = Mass::new(data, Some(orbits));
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn predict(&mut self, dt: f64) {
        for mass in &mut self.masses {
            mass.save();
        }

        let count = 44;
        for _ in 0..count {
            for _ in 0..5 {
                self.frame(dt);
            }

            for mass in &mut self.masses {
                mass.prediction.push(mass.position);
                if mass.prediction.len() > count {
                    mass.prediction.remove(0);
                }
            }
        }
        for mass in &mut self.masses {
            mass.restore();
        }
    }

    pub fn frame(&mut self, seconds_per_frame: f64) {
        // each mass drags each other mass, except itselfes
        for mass_index in 0..self.masses.len() {
            // todo: don't clone
            let mass = self.masses[mass_index].clone();
            for dragged in &mut self.masses {
                // todo: no string compare
                if mass.name != dragged.name {
                    dragged.dragged_by(&mass);
                }
            }
        }

        for mass in &mut self.masses {
            mass.frame_move(seconds_per_frame);
        }
    }

    pub fn draw(&mut self) {
        //??? _ = self.masses.iter().map(|m| m.draw());

        for mass in &mut self.masses {
            mass.draw();
        }
    }
}
