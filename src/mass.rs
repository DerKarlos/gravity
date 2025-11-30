use crate::vec_space::*;
use macroquad::prelude::*;

// Parameter
pub const FRAME_TIME: f64 = 0.02; // 20ms = 50Hz
const PREDICT_COUNT: usize = 500;
const SECONDS_PER_ORBIT: f64 = 10.;

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
pub const MYSTIC_G_FACT: f64 = 2.; // 1 or as in the old code 2 ??? And why?

// ------------------- SI UNIT VALUE KONVERT OPTIONS  -------------------

// distances
pub fn km(km: f64) -> f64 {
    km * 1000.
}
pub fn au(au: f64) -> f64 {
    au * M_AU
}
pub fn kg(kg: f64) -> f64 {
    kg
}

// masses (wheight)
pub fn mass_earth(earth: f64) -> f64 {
    earth * 5.974e24
}
pub fn mass_sol(sol: f64) -> f64 {
    sol * 1.989e30
}

// ------------------- MASS STRUCT/CLASS -------------------

#[derive(Debug, Default, Clone, Copy)]
pub struct MassData<'a> {
    name: &'a str,
    color: Color,
    diameter: f64,
    mass: f64,
    orbit_radius: f64,
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
    ) -> MassData {
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
    ) -> MassData {
        MassData::ellipse(name, color, diameter, mass, orbit_radius, 0.0)
    }

    pub fn fixstar(name: &str, color: Color, diameter: f64, mass: f64) -> MassData {
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
    predict_index: usize,
}

impl Mass {
    // "Static" constants

    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        let position = VecSpace::new(0.0, data.orbit_radius);
        let velocity = VecSpace::ZERO;
        let acceleration = VecSpace::ZERO;

        let mut mass = Mass {
            _name: data.name.to_string(),
            color: data.color,
            diameter: data.diameter,
            mass: data.mass,
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
            predict_index: 0,
        };

        if orbits.is_some() {
            Self::mass_v_orbit(&mut mass, &mut orbits.unwrap(), data.excentricity);
        }

        for _ in 0..PREDICT_COUNT {
            mass.prediction.push(position);
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
        let velocity = (MYSTIC_G_FACT * GRAVITY_CONSTANT_OF_EARTH * both_masses / radius).sqrt()
            * (1. - excentriticy);
        mass.velocity += VecSpace::new(velocity / both_masses * other.mass * signum, 0.);
        other.velocity += VecSpace::new(-velocity / both_masses * mass.mass * signum, 0.);
    }

    fn _v_orbit(central_mass: f64, radius: f64) -> f64 {
        (MYSTIC_G_FACT * GRAVITY_CONSTANT_OF_EARTH * central_mass / radius).sqrt()
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

            let acceleration =
                other_mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH * MYSTIC_G_FACT;
            let acceleration_vector = distance_vector * acceleration;

            self.acceleration += acceleration_vector;
        }
    }

    pub fn frame_move(&mut self, simulated_seconds_per_frame: f64) {
        self.velocity += self.acceleration * simulated_seconds_per_frame;
        self.position += self.velocity * simulated_seconds_per_frame;
        self.acceleration.set_zero();
    }

    pub fn draw(&self, masses: &Masses) {
        // sqrt(sqrt()) scaling like Kotlin code
        let mut size = ((self.diameter / M_AU).sqrt().sqrt() / 2.0 * DRAW_FACT) as i32;
        size = size.clamp(DRAW_MIN, DRAW_MAX);

        let screen_pos = masses.scale(&self.position);
        draw_circle(
            screen_pos.x() as f32,
            screen_pos.y() as f32,
            size as f32,
            self.color,
        );

        let mut last_pos = screen_pos;
        for position in &self.prediction {
            let this_pos = masses.scale(position);

            if false {
                draw_line(
                    last_pos.x() as f32,
                    last_pos.y() as f32,
                    this_pos.x() as f32,
                    this_pos.y() as f32,
                    0.1,
                    WHITE,
                );
            } else {
                draw_rectangle(this_pos.x() as f32, this_pos.y() as f32, 1., 1., WHITE);
            }

            last_pos = this_pos;
        }
    }
}

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Masses {
    text: String,
    pub case: i16,
    masses: Vec<Mass>,
    z_view: f64,
    pub maximal_orbit: f64,
    pub seconds_per_orbit: f64,
    pub simulated_seconds: f64,
    pub simulated_seconds_per_secound: f64,
    pub simulated_seconds_per_frame: f64,
    pub planing_mode: bool,
    start_time: f64,
    burn_time: f64,
}

impl Masses {
    pub fn new(case: i16) -> Masses {
        Masses {
            text: String::new(),
            case,
            masses: Vec::new(),
            z_view: 1.2,
            maximal_orbit: 0.0,
            seconds_per_orbit: SECONDS_PER_ORBIT,
            simulated_seconds: 0.0,
            simulated_seconds_per_secound: 0.0,
            simulated_seconds_per_frame: 0.0,
            planing_mode: false,
            start_time: 0.0,
            burn_time: 0.0,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn add_at_place(&mut self, data: &MassData) -> usize {
        let mass = Mass::new(data, None);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        let orbits = &mut self.masses[orbits];
        self.maximal_orbit = data.orbit_radius.max(self.maximal_orbit);
        self.z_view = 1.1 * self.maximal_orbit / M_AU;
        //println!("max orbit: {}", self.maximal_orbit);
        let mass = Mass::new(data, Some(orbits));
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn ship_accelerate(&mut self, acceleration: f64) {
        let ship_index = self.masses.len() - 1;
        let ship = &mut self.masses[ship_index];
        ship.accelerate(acceleration);
    }

    pub fn toggle_planing_mode(&mut self) {
        self.planing_mode = !self.planing_mode;
        if self.planing_mode {
            //let x = 1e4;
            let y = 1e3;
            self.start_time = self.simulated_seconds + y * 2.;
            self.burn_time = y;
        }
    }

    pub fn planing_start_time(&mut self, set: f64) {
        self.start_time += set * 0.5;
    }

    pub fn planing_burn_time(&mut self, set: f64) {
        self.burn_time *= 1. + set * 0.0002;
    }

    pub fn predict(&mut self) {
        for mass in &mut self.masses {
            mass.save();
        }

        for _ in 0..PREDICT_COUNT {
            self.simulate();
            self.simulated_seconds += self.simulated_seconds_per_frame;

            for mass in &mut self.masses {
                mass.prediction[mass.predict_index] = mass.position;
                mass.predict_index = (mass.predict_index + 1) % PREDICT_COUNT;
                //mass.prediction.push(mass.position);
                //if mass.prediction.len() > count {
                //    mass.prediction.remove(0);
                //}
            }
        }

        for mass in &mut self.masses {
            mass.restore();
        }
    }

    pub fn simulate(&mut self) {
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
        let start = self.start_time;
        let end = self.start_time + self.burn_time;
        if self.simulated_seconds > start && self.simulated_seconds < end {
            ship.accelerate(1.);
        }

        for mass in &mut self.masses {
            mass.frame_move(self.simulated_seconds_per_frame);
        }

        self.simulated_seconds += self.simulated_seconds_per_frame;
    }

    pub fn scale(&self, position: &VecSpace) -> VecSpace {
        let window_center: VecSpace =
            VecSpace::new((WINDOW_WIDTH / 2) as f64, (WINDOW_HEIGHT / 2) as f64);
        *position * (PIXEL as f64 / self.z_view / M_AU) + window_center
    }

    pub fn draw(&mut self) {
        //??? _ = self.masses.iter().map(|m| m.draw());
        draw_text(self.text.as_str(), 20.0, 20.0, 30.0, DARKGRAY);

        for mass in &self.masses {
            mass.draw(&self);
        }
    }
}
