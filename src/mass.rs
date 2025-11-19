use crate::vec_space::*;
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 680; // ??? calculate frame

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)
pub const M_AU: f64 = 149_597_870_700.0; // m per Astronomic Unit
pub const MAX_GRAVITY_DISTANCE: f64 = 1e38; // [AE]
pub const DRAW_FACT: f64 = 200.0;
pub const DRAW_MIN: i32 = 3;
pub const DRAW_MAX: i32 = 200;
pub const SECONDS_PER_DAY: f64 = 60. * 60. * 24.;
pub const SECONDS_PER_YEAR: f64 = SECONDS_PER_DAY * 365.25;

// Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
// The smallest extend of the window counts as visible screen range
const PIXEL: i32 = WINDOW_HEIGHT / 2; // todo: do it dynamic!

// ------------------- SI UNIT VALUE KONVERT OPTIONS  -------------------

#[derive(Debug, Default)]
pub enum Si {
    // Distance
    #[default]
    Null,
    _M(f64),
    Km(f64),
    Au(f64),
    _LightYear(f64),
    // Mass
    Kg(f64),
    Earth(f64),
    Sol(f64),
    // Time
    _MilliSecound(f64),
    _Secound(f64),
    _Year(f64),
}

pub fn si_into(value: &Si) -> f64 {
    match value {
        Si::Null => 0.0,
        Si::_M(m) => *m,
        Si::Km(m) => m * 1000.,
        Si::Au(ae) => ae * M_AU,
        Si::_LightYear(ly) => ly * 0.0,

        Si::Kg(kg) => *kg,
        Si::Earth(e) => e * 5.974e24,
        Si::Sol(s) => s * 1.989e30,

        Si::_MilliSecound(ms) => ms / 1000.0,
        Si::_Secound(s) => *s,
        Si::_Year(y) => y * SECONDS_PER_YEAR,
    }
}

// ------------------- MASS STRUCT/CLASS -------------------

#[derive(Debug, Default)]
pub struct MassData<'a> {
    name: &'a str,
    color: Color,
    diameter: Si,
    mass: Si,
    radius: Si,
    excentricity: f64,
}

impl<'a> MassData<'a> {
    // "Static" constants

    pub fn ellipse(
        name: &str,
        color: Color,
        diameter: Si,
        mass: Si,
        radius: Si,
        excentricity: f64,
    ) -> MassData {
        MassData {
            name,
            color,
            diameter,
            mass,
            radius,
            excentricity,
        }
    }

    pub fn orbiter(name: &str, color: Color, diameter: Si, mass: Si, radius: Si) -> MassData {
        MassData::ellipse(name, color, diameter, mass, radius, 0.0)
    }

    pub fn fixstar(name: &str, color: Color, diameter: Si, mass: Si) -> MassData {
        MassData::ellipse(name, color, diameter, mass, Si::Au(0.0), 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Mass {
    position: VecSpace,
    velocity: VecSpace,
    saved_position: VecSpace,
    saved_velocity: VecSpace,
    acceleration: VecSpace,
    color: Color,
    _name: String,
    mass: f64,
    diameter: f64,
    prediction: Vec<VecSpace>,
}

impl Mass {
    // "Static" constants

    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        let position = VecSpace::new(0.0, si_into(&data.radius));
        let velocity = VecSpace::ZERO;
        let acceleration = VecSpace::ZERO;

        let mut mass = Mass {
            _name: data.name.to_string(),
            color: data.color,
            diameter: si_into(&data.diameter),
            mass: si_into(&data.mass),
            saved_position: VecSpace::ZERO,
            saved_velocity: VecSpace::ZERO,
            position: if orbits.is_some() {
                position
            } else {
                VecSpace::ZERO
            },
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
        let signum = if mass.position.y() > 0.0 { 1.0 } else { -1.0 };
        mass.position += other.position;
        mass.velocity += other.velocity;
        let radius = (other.position - mass.position).length();

        let both_masses = mass.mass + other.mass;
        let velocity =
            (GRAVITY_CONSTANT_OF_EARTH * both_masses / radius).sqrt() * (1. - excentriticy);
        mass.velocity += VecSpace::new(velocity / both_masses * other.mass * signum, 0.);
        other.velocity += VecSpace::new(-velocity / both_masses * mass.mass * signum, 0.);
    }

    fn _v_orbit(central_mass: f64, radius: f64) -> f64 {
        (GRAVITY_CONSTANT_OF_EARTH * central_mass / radius).sqrt()
    }

    pub fn get_drag_values(&mut self) -> (f64, VecSpace) {
        (self.mass, self.position)
    }

    pub fn save(&mut self) {
        self.saved_position = self.position;
        self.saved_velocity = self.velocity;
    }

    pub fn restore(&mut self) {
        self.position = self.saved_position;
        self.velocity = self.saved_velocity;
    }

    pub fn accelerate(&mut self, acceleration: f64) {
        let direction = self.velocity.normalized();
        self.acceleration += direction * acceleration * 1.;
    }

    pub fn dragged_by(&mut self, (other_mass, other_position): (f64, VecSpace)) {
        if other_mass == 0.0 {
            return; // don’t drag zero-mass objects
        }

        let mut distance_vector = other_position - self.position;
        let distance = distance_vector.length();

        if distance < MAX_GRAVITY_DISTANCE * M_AU {
            distance_vector.normalize();

            let acceleration = other_mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
            let acceleration_vector = distance_vector * acceleration;

            self.acceleration += acceleration_vector;
        }
    }

    pub fn frame_move(&mut self, dt: f64) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration.set_zero();
    }

    pub fn draw(&self, masses: &Masses) {
        // sqrt(sqrt()) scaling like Kotlin code
        let mut size = ((self.diameter / M_AU).sqrt().sqrt() / 2.0 * DRAW_FACT) as i32;
        size = size.clamp(DRAW_MIN, DRAW_MAX);

        let screen_pos = scale(&self.position, masses.z_view);
        draw_circle(
            screen_pos.x() as f32,
            screen_pos.y() as f32,
            size as f32,
            self.color,
        );

        let mut last = screen_pos;
        for position in &self.prediction {
            let this = scale(position, masses.z_view);

            if false {
                draw_line(
                    last.x() as f32,
                    last.y() as f32,
                    this.x() as f32,
                    this.y() as f32,
                    0.1,
                    WHITE,
                );
            } else {
                draw_rectangle(this.x() as f32, this.y() as f32, 1., 1., WHITE);
            }

            last = this;
        }
    }
}

fn scale(position: &VecSpace, z_view: f64) -> VecSpace {
    let window_center: VecSpace =
        VecSpace::new((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
    *position * (PIXEL as f64 / z_view / M_AU) + window_center
}

// ------------------- MASSES STRUCT/CLASS -------------------
pub struct Masses {
    masses: Vec<Mass>,
    z_view: f64,
    pub maximal_orbit: f64,
}

impl Masses {
    pub fn new() -> Masses {
        Masses {
            masses: Vec::new(),
            z_view: 1.2,
            maximal_orbit: 0.0,
        }
    }

    pub fn add_at_place(&mut self, data: &MassData) -> usize {
        let mass = Mass::new(data, None);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        let orbits = &mut self.masses[orbits];
        self.maximal_orbit = si_into(&data.radius).max(self.maximal_orbit);
        self.z_view = 1.1 * self.maximal_orbit / M_AU;
        println!("{}", self.maximal_orbit);
        let mass = Mass::new(data, Some(orbits));
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn ship_accelerate(&mut self, acceleration: f64) {
        let ship_index = self.masses.len() - 1;
        let ship = &mut self.masses[ship_index];
        ship.accelerate(acceleration);
    }

    pub fn predict(&mut self, simulated_seconds_per_frame: f64, mut simulated_seconds: f64) {
        for mass in &mut self.masses {
            mass.save();
        }

        let count = 1000;
        for _ in 0..count {
            self.simulate(simulated_seconds_per_frame, simulated_seconds);
            simulated_seconds += simulated_seconds_per_frame;

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

    pub fn simulate(&mut self, seconds_per_frame: f64, simulated_seconds: f64) {
        // each mass drags each other mass, except itselfes
        for mass_index in 0..self.masses.len() {
            let drag_values = self.masses[mass_index].get_drag_values();

            for dragged_index in 0..self.masses.len() {
                if mass_index == dragged_index {
                    continue;
                }
                let dragged = &mut self.masses[dragged_index];
                dragged.dragged_by(drag_values);
            }
        }

        let ship_index = self.masses.len() - 1;
        let ship = &mut self.masses[ship_index];
        let x = 1e4;
        let y = x + 1e3;
        draw_text(
            (x / SECONDS_PER_DAY).to_string().as_str(),
            20.0,
            60.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            (simulated_seconds / SECONDS_PER_DAY).to_string().as_str(),
            20.0,
            80.0,
            30.0,
            DARKGRAY,
        );
        if simulated_seconds > x && simulated_seconds < y {
            ship.accelerate(-1.);
        }

        for mass in &mut self.masses {
            mass.frame_move(seconds_per_frame);
        }
    }

    pub fn draw(&mut self) {
        //??? _ = self.masses.iter().map(|m| m.draw());

        for mass in &self.masses {
            mass.draw(&self);
        }
    }
}
