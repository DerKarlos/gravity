use crate::vec_space::*;
use macroquad::prelude::*;

///// Parameter /////

// About like the framerate in Hz, but will be checked and repeated if needed
pub const SIMULATION_STEPS_PER_SECOND: f64 = 50.;

pub const PREDICT_COUNT: usize = 900;
pub const DEFAULT_SECONDS_PER_ORBIT: f64 = 10.; // default for earth!!! weg???

pub const WINDOW_WIDTH: f32 = 1000.;
pub const WINDOW_HEIGHT: f32 = 680.; // ??? calculate frame

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)
pub const DRAW_FACT: f64 = 200.0;
pub const DRAW_MIN: i32 = 3;
pub const DRAW_MAX: i32 = 200;
//b const MAX_GRAVITY_DISTANCE: f64 = 1e38; // [AE]

// Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
// The smallest extend of the window counts as visible screen range
const PIXEL: i32 = WINDOW_HEIGHT as i32 / 2; // todo: do it dynamic!

// ------------------- SI UNIT VALUE KONVERT OPTIONS  -------------------

// distances
pub fn km(km: f64) -> f64 {
    km * 1000.
}
pub fn au(au: f64) -> f64 {
    au * 149_597_870_700.0 // m per Astronomic Unit
}
pub fn one_au() -> f64 {
    au(1.)
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
    _name: String,
    mass: f64,
    diameter: f64,
    orbit_time: f64,
    color: Color,
    acceleration: VecSpace,
    velocity: VecSpace,
    position: VecSpace,
    positions: [VecSpace; PREDICT_COUNT],
}

impl Mass {
    // "Static" constants

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

        // calculate the real time for one orbital period in seconds
        2.0 * std::f64::consts::PI
            * (radius.powi(3) / (GRAVITY_CONSTANT_OF_EARTH * (mass.mass + other.mass))).sqrt()
    }

    // only used for a ship (no mass)
    pub fn accelerate(&mut self, acceleration: f64) {
        let direction = self.velocity.normalized();
        self.acceleration += direction * acceleration * 1.;
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

    pub fn move_seconds(&mut self, dt_sim: f64, pi: usize) {
        self.velocity += self.acceleration * dt_sim;
        self.position += self.velocity * dt_sim;
        self.acceleration.set_zero();
        self.positions[pi] = self.position;
    }

    pub fn draw(&self, masses: &Masses, positions_index: usize) {
        // sqrt(sqrt()) scaling like Kotlin code
        let mut size = ((self.diameter / one_au()).sqrt().sqrt() / 2.0 * DRAW_FACT) as i32;
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

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Masses {
    text: String,
    pub case: i16,
    masses: Vec<Mass>,
    z_view: f64,
    pub positions_index: usize,
    pub maximal_orbit_radius: f64,
    pub maximal_orbit_time: f64,
    pub seconds_per_orbit: f64,
    pub simulated_seconds: f64,
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
            positions_index: 0,
            maximal_orbit_radius: 0.0,
            maximal_orbit_time: 0.0,
            seconds_per_orbit: DEFAULT_SECONDS_PER_ORBIT,
            simulated_seconds: 0.0,
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
        self.maximal_orbit_radius = data.orbit_radius.max(self.maximal_orbit_radius);
        self.z_view = 1.1 * self.maximal_orbit_radius / one_au();
        //println!("max orbit: {}", self.maximal_orbit);
        let mass = Mass::new(data, Some(orbits));
        self.maximal_orbit_time = self.maximal_orbit_time.max(mass.orbit_time);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    // initially simulate all the future positinos
    pub fn simulate_positions(&mut self) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.simulated_seconds_per_frame =
            self.maximal_orbit_time / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;

        for _ in 0..PREDICT_COUNT {
            self.simulate(self.simulated_seconds_per_frame);
        }
    }

    pub fn simulate_next_position(&mut self) {
        self.simulate(self.simulated_seconds_per_frame);
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

    pub fn simulate(&mut self, dt_sim: f64) {
        // each mass drags each other mass, except itselfes
        for i in 0..self.masses.len() {
            // let drag_values = self.masses[i].get_drag_values();
            for j in (i + 1)..self.masses.len() {
                let (left, right) = self.masses.split_at_mut(j);

                let a = &mut left[i];
                let b = &mut right[0];
                a.drag(b);
                b.drag(a);
            }
        }

        // each mass drags each other mass, except itselfes
        //for i in 0..self.masses.len() {
        //    for j in (i + 1)..self.masses.len() {
        //        let (left, right) = self.masses.split_at_mut(j);
        //
        //        let a = &mut left[i];
        //        let b = &mut right[0];
        //        a.drag(b);
        //        b.drag(a);
        //    }
        //}

        for mass in &mut self.masses {
            mass.move_seconds(dt_sim, self.positions_index); // 2.0e4
        }
        self.positions_index += 1;
        self.positions_index %= PREDICT_COUNT;

        let ship_index = self.masses.len() - 1;
        let ship = &mut self.masses[ship_index];
        let start = self.start_time;
        let end = self.start_time + self.burn_time;
        if self.simulated_seconds > start && self.simulated_seconds < end {
            ship.accelerate(1.);
        }

        //for mass in &mut self.masses {
        //    mass.frame_move(self.simulated_seconds_per_frame);
        //}

        self.simulated_seconds += self.simulated_seconds_per_frame;
    }

    pub fn scale(&self, position: &VecSpace) -> VecSpace {
        // return f32 (x,y) ???
        let window_center: VecSpace =
            VecSpace::new(WINDOW_WIDTH as f64 / 2., WINDOW_HEIGHT as f64 / 2.);
        *position * (PIXEL as f64 / self.z_view / one_au()) + window_center
    }

    pub fn draw(&mut self) {
        //??? _ = self.masses.iter().map(|m| m.draw());

        draw_text(
            format!("{} {}", self.text, self.positions_index).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );

        for mass in &self.masses {
            mass.draw(&self, self.positions_index);
        }
    }
}
