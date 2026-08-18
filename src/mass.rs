use crate::simulation::*;
use crate::vec_space::*;
use macroquad::prelude::*;

///// Parameter /////

// About like the framerate in Hz, but will be checked and repeated if needed
pub const SIMULATION_STEPS_PER_SECOND: f64 = 50.;
pub const SIMULATION_STEP_TIME: f64 = 1. / SIMULATION_STEPS_PER_SECOND;

pub const PREDICT_COUNT: usize = 1000;
pub const DEFAULT_SECONDS_PER_ORBIT: f64 = 10.; // default for earth!!! weg???

pub const WINDOW_WIDTH: f32 = 1000.;
pub const WINDOW_HEIGHT: f32 = 680.; // ??? calculate frame

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)
pub const DRAW_FACT: f64 = 5.;
pub const DRAW_MIN: i32 = 3;
pub const DRAW_MAX: i32 = 200;
//b const MAX_GRAVITY_DISTANCE: f64 = 1e38; // [AE]

// Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
// The smallest extend of the window counts as visible screen range
pub const MAX_PIXEL_FROM_CENTER: i32 = WINDOW_HEIGHT as i32 / 2; // todo: do it dynamic!

// ------------------- SI UNIT VALUE KONVERT OPTIONS  -------------------

// distances
pub fn km(km: f64) -> f64 {
    km * 1000.
}
pub fn au(au: f64) -> f64 {
    au * 149_597_870_700.0 // m per Astronomic Unit
}

// masses (wheight)
pub fn kg(kg: f64) -> f64 {
    kg
}
pub fn mass_earth(earth: f64) -> f64 {
    earth * 5.974e24
}
pub fn mass_sol(sol: f64) -> f64 {
    sol * 1.989e30
}

// ------------------- MASS-DATA STRUCT/CLASS -------------------

#[derive(Debug, Default, Clone, Copy)]
pub struct MassData<'a> {
    name: &'a str,
    color: Color,
    diameter: f64,
    mass: f64,
    pub orbit_radius: f64,
    excentricity: f64,
}

impl<'a> MassData<'a> {
    // "Static" constants

    pub fn ellipse(
        name: &str,
        color: Color,
        diameter: f64,
        mass: f64,
        orbit_radius: f64,
        excentricity: f64,
    ) -> MassData<'_> {
        MassData {
            name,
            color,
            diameter,
            mass,
            orbit_radius,
            excentricity,
        }
    }

    pub fn orbiter(
        name: &str,
        color: Color,
        diameter: f64,
        mass: f64,
        orbit_radius: f64,
    ) -> MassData<'_> {
        MassData::ellipse(name, color, diameter, mass, orbit_radius, 0.0)
    }

    pub fn fixstar(name: &str, color: Color, diameter: f64, mass: f64) -> MassData<'_> {
        MassData::ellipse(name, color, diameter, mass, 0.0, 0.0)
    }
    pub fn _mul_orbit_radius(&mut self, fakt: f64) {
        self.orbit_radius = self.orbit_radius * fakt;
    }
    pub fn multiplied_orbit_radius(&self, fakt: f64) -> Self {
        let mut ret = self.clone();
        ret.orbit_radius *= fakt;
        ret
    }
}

// =================== MASS STRUCT/CLASS ===================

#[derive(Debug, Clone)]
pub struct Mass {
    _name: String, // why not &str ???
    mass: f64,
    diameter: f64,
    pub orbit_time: f64,
    color: Color,
    acceleration: VecSpace,
    pub velocity: VecSpace,
    pub position: VecSpace,
    pub positions: [VecSpace; PREDICT_COUNT],
}

impl Mass {
    pub fn zero() -> Mass {
        Mass {
            _name: String::from("ZERO"),
            mass: 0.,
            diameter: 0.,
            orbit_time: 0.,
            color: BLACK,
            acceleration: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            position: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        }
    }

    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        let position = VecSpace::new(data.orbit_radius, 0.0);
        let velocity = VecSpace::ZERO;
        let acceleration = VecSpace::ZERO;

        let mut mass = Mass {
            _name: data.name.to_string(),
            color: data.color,
            diameter: data.diameter,
            orbit_time: 0.,
            mass: data.mass,
            position: if orbits.is_some() {
                position
            } else {
                VecSpace::ZERO
            },
            velocity,
            acceleration,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        };

        if orbits.is_some() {
            mass.orbit_time = Self::set_v_orbit(&mut mass, &mut orbits.unwrap(), data.excentricity);
        }

        mass.positions[0] = position;
        return mass;
    }

    /// Computes orbital velocity for a circular orbit
    /// around a body with `central_mass` at distance `radius` (in meters)

    fn set_v_orbit(mass: &mut Mass, other: &mut Mass, excentriticy: f64) -> f64 {
        let signum = if mass.position.y() > 0.0 { 1.0 } else { -1.0 };
        mass.position += other.position;
        mass.velocity += other.velocity;
        let radius = (other.position - mass.position).length();

        let both_masses = mass.mass + other.mass;
        let velocity =
            (GRAVITY_CONSTANT_OF_EARTH * both_masses / radius).sqrt() * (1. - excentriticy);
        mass.velocity += VecSpace::new(0., -velocity / both_masses * other.mass * signum);
        other.velocity += VecSpace::new(0., velocity / both_masses * mass.mass * signum);
        // ??? Could we also move the other mass to get the common rotation point in the center?

        // calculate the real time for one orbital period in seconds
        2.0 * std::f64::consts::PI
            * (radius.powi(3) / (GRAVITY_CONSTANT_OF_EARTH * (mass.mass + other.mass))).sqrt()
    }

    pub fn ship_accelerate(&mut self, acceleration: f64) {
        let direction = self.velocity.normalized();
        self.acceleration += direction * acceleration;
    }

    pub fn drag(&self, other: &mut Mass) {
        let mut distance_vector = self.position - other.position;
        let distance = distance_vector.length();
        distance_vector.normalize();

        // F = force (N) : m = mass (kg) / r² = distance² (m²) * G = 6.67430 × 10⁻¹¹ m³/(kg·s²)
        let acceleration = self.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
        let acceleration_vector = distance_vector * acceleration;

        other.acceleration += acceleration_vector;
    }

    pub fn move_seconds(&mut self, simulated_seconds_per_step: f64, positions_index: usize) {
        self.velocity += self.acceleration * simulated_seconds_per_step;
        self.position += self.velocity * simulated_seconds_per_step;
        self.acceleration.set_zero();
        self.positions[positions_index] = self.position;
    }

    pub fn _move_ship(&mut self, simulated_seconds_per_step: f64, _positions_index: usize) {
        self.velocity += self.acceleration * simulated_seconds_per_step;
        self.position += self.velocity * simulated_seconds_per_step;
        self.acceleration.set_zero();
    }

    pub fn draw(&self, masses: &Simulation, positions_index: usize) {
        // visible size not real and less proportional to avoid big differences
        let mut size = (self.diameter.sqrt().sqrt() / DRAW_FACT * masses.z_view) as i32;
        size = size.clamp(DRAW_MIN, DRAW_MAX);

        let screen_pos = masses.scale(&self.positions[positions_index]);
        draw_circle(
            screen_pos.x() as f32,
            screen_pos.y() as f32,
            size as f32,
            self.color,
        );
        //println!("x/y {}/{}", screen_pos.x() as f32, screen_pos.y() as f32);

        let mut last_pos = screen_pos;
        for position in &self.positions {
            let this_pos = masses.scale(position);

            if false {
                draw_line(
                    last_pos.x() as f32,
                    last_pos.y() as f32,
                    this_pos.x() as f32,
                    this_pos.y() as f32,
                    0.1,
                    self.color,
                );
            } else {
                draw_rectangle(this_pos.x() as f32, this_pos.y() as f32, 1., 1., self.color);
            }

            last_pos = this_pos;
        }
    }
}
