//! Collection and instances of masses, like sun moon and stars
//! and mass related values, calculated of them

use crate::canvas::*;
use crate::simulation::*;
use crate::vec_space::*;
use macroquad::color::GREEN;
use macroquad::prelude::{Color, RED};

//////////////////// Parameter ////////////////////

pub const PREDICT_COUNT: usize = 1000;

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)

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

    pub fn mul_radius(&self, fakt: f64) -> Self {
        let mut ret = *self; // = self.clone();
        ret.orbit_radius *= fakt;
        ret
    }
    pub fn _set_radius(&self, val: f64) -> Self {
        let mut ret = *self; // = self.clone();
        ret.orbit_radius = val;
        ret
    }
}

// =================== MASS STRUCT/CLASS ===================

#[derive(Debug, Clone)]
pub struct Mass {
    _name: String, // why not &'static str ???
    index: usize,
    mass: f64,
    diameter: f64,
    orbits: usize,
    lagrange: i8,
    color: Color,
    acceleration: VecSpace,
    velocity: VecSpace,
    position: VecSpace,
    positions: [VecSpace; PREDICT_COUNT],
}

impl Mass {
    pub fn zero() -> Mass {
        Mass {
            _name: String::from("ZERO"),
            index: 0,
            mass: 0.,
            diameter: 0.,
            orbits: 0,
            lagrange: 0,
            color: GREEN,
            acceleration: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            position: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        }
    }

    // pub for new ship
    pub fn new(data: &MassData, orbits: Option<&mut Mass>, index: usize) -> (Mass, f64) {
        let mut mass = Mass {
            _name: data.name.to_string(),
            index,
            color: data.color,
            diameter: data.diameter,
            orbits: 0,
            lagrange: 0,
            mass: data.mass,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            acceleration: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        };

        let mut orbit_seconds: f64 = 0.;
        if let Some(orbited_mass) = orbits {
            mass.orbits = orbited_mass.index;
            mass.position += VecSpace::new(data.orbit_radius, 0.); // + orbited_mass.position; done by set_v_orbit
            orbit_seconds = Self::set_v_orbit(&mut mass, orbited_mass, data.excentricity);
        };

        mass.positions[0] = mass.position;
        (mass, orbit_seconds)
    }

    pub fn position(&self) -> VecSpace {
        self.position
    }

    pub fn set_position(&mut self, val: VecSpace) {
        self.position = val;
        self.positions[0] = val; // always 0 ??? Only used for ship 20 Oct
    }

    pub fn velocity(&self) -> VecSpace {
        self.velocity
    }

    pub fn set_velocity(&mut self, val: VecSpace) {
        self.velocity = val;
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

        // calculate the real simulated seconds for one orbital period in seconds
        2.0 * std::f64::consts::PI
            * (radius.powi(3) / (GRAVITY_CONSTANT_OF_EARTH * (mass.mass + other.mass))).sqrt()
    }

    pub fn accelerate_ahead(&mut self, acceleration: f64) {
        let direction = self.velocity._normalized();
        self.acceleration += direction * acceleration;
    }

    pub fn ship_accelerate_vec(&mut self, acceleration_vector: VecSpace) {
        self.acceleration += acceleration_vector;
    }

    fn drag(&self, other: &mut Mass) {
        let mut distance_vector = self.position - other.position;
        let distance = distance_vector.length();
        distance_vector.normalize();

        // F = force (N) : m = mass (kg) / r² = distance² (m²) * G = 6.67430 × 10⁻¹¹ m³/(kg·s²)
        let acceleration = self.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
        let acceleration_vector = distance_vector * acceleration;

        other.acceleration += acceleration_vector;
    }

    fn get_drag_for_that_position(&self, position: VecSpace, position_index: usize) -> VecSpace {
        let mut distance_vector = self.positions[position_index] - position;
        let distance = distance_vector.length();
        distance_vector.normalize();

        // F = force (N) : m = mass (kg) / r² = distance² (m²) * G = 6.67430 × 10⁻¹¹ m³/(kg·s²)
        let acceleration = self.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
        distance_vector * acceleration
    }

    pub fn move_seconds(&mut self, seconds: f64, positions_index: usize) {
        self.velocity += self.acceleration * seconds;
        self.position += self.velocity * seconds;
        self.positions[positions_index] = self.position;
        self.acceleration.set_zero();
    }

    // do it by thread_local ?
    pub fn draw(&self, positions_index: usize, predict_show: usize, predict_count: f64) {
        // Invisible masses (also ship::ZERO)
        if self.diameter == 0. {
            return;
        }
        let index = (positions_index + predict_show) % PREDICT_COUNT;
        draw_circle(
            self.positions[index],
            // visible size not real and less proportional to avoid big differences
            self.diameter.sqrt().sqrt(),
            self.color,
        );
        //println!("x/y {}/{}", screen_pos.x() as f32, screen_pos.y() as f32);

        for index in 0..predict_count as usize {
            let index = (index + positions_index) % PREDICT_COUNT;
            let position = self.positions[index];
            draw_rectangle(position, self.color);
        }
    }
}

// =================== M A S S E S STRUCT/CLASS ===================

// masses-values, calcualted while creating the masses
#[derive(Debug, Clone)]
pub struct Masses {
    masses: Vec<Mass>,
    // At this index: draw oldest position, write new position, increment after write
    positions_index: usize,
    // Default 10s or set by the actual scene
    maximal_orbit_seconds: f64,
    // Calculated by the masses. Also needed and copied to the canvas.
    maximal_orbit_radius: f64,
    pub predict_count: f64,
    predict_show: f64,
}

impl Masses {
    pub fn new() -> Masses {
        Masses {
            masses: Vec::new(),
            positions_index: 0,
            maximal_orbit_seconds: 1.,
            maximal_orbit_radius: 1.,
            predict_count: PREDICT_COUNT as f64,
            predict_show: 0.,
        }
    }

    pub fn positions_index(&self) -> usize {
        self.positions_index
    }

    pub fn maximal_orbit_seconds(&self) -> f64 {
        self.maximal_orbit_seconds
    }

    pub fn mul_predict_count(&mut self, fakt: f64) {
        self.predict_count *= fakt;
        if self.predict_count > PREDICT_COUNT as f64 {
            self.predict_count = PREDICT_COUNT as f64;
        };
    }

    pub fn set_predict_show(&mut self, val: f64) {
        self.predict_show = val;
    }

    pub fn add_at_place(&mut self, data: &MassData) -> usize {
        let index = self.masses.len();
        let (mass, _) = Mass::new(data, None, index);
        println!("add_at_place: {}", index);
        self.masses.push(mass);
        index
    }

    pub fn set_in_orbit(&mut self, data: &MassData, orbits_index: usize) -> Mass {
        let index = self.masses.len();
        println!("set_in_orbit: {} {}", index, orbits_index);
        let orbits = &mut self.masses[orbits_index];
        let (mut mass, orbit_seconds) = Mass::new(data, Some(orbits), index);
        mass.orbits = orbits_index;
        self.maximal_orbit_radius = data.orbit_radius.abs().max(self.maximal_orbit_radius);
        println!("max orbit: {}", self.maximal_orbit_radius);
        self.maximal_orbit_seconds = self.maximal_orbit_seconds.max(orbit_seconds);
        mass
    }

    pub fn add_in_orbit(&mut self, data: &MassData, orbits: usize, lagrange: i8) -> usize {
        let index = self.masses.len();
        let mut mass = self.set_in_orbit(data, orbits);
        mass.lagrange = lagrange;
        self.masses.push(mass);
        index
    }

    pub fn orbit_relativ_position(&self, mass_index: usize, predict_show: usize) -> VecSpace {
        let orbited_index = self.masses[mass_index].orbits;
        let orbited_position = self.masses[orbited_index].position();
        let mass_position = self.masses[mass_index].positions[predict_show];
        let relativ = mass_position - orbited_position;
        // println!("{:?} {:?}", mass_position, orbited_position);
        // relativ.to_angle()
        relativ
    }

    pub fn set_radius(&self) {
        draw_set_maximal_orbit_radius(self.maximal_orbit_radius);
    }

    pub fn get_position(&self, index: usize) -> VecSpace {
        self.masses[index].position
    }

    pub fn drag_at_position(&self, position: VecSpace, index: usize) -> VecSpace {
        let mut acceleration = VecSpace::ZERO;
        for mass in &self.masses {
            acceleration += mass.get_drag_for_that_position(position, index)
        }
        acceleration
    }

    pub fn drag_and_move(&mut self, seconds: f64) {
        // First drag, sedound move

        // Each mass drags each other mass, except itselfes
        for i in 0..self.masses.len() {
            for j in (i + 1)..self.masses.len() {
                let (left, right) = self.masses.split_at_mut(j);

                let a = &mut left[i];
                let b = &mut right[0];
                a.drag(b);
                b.drag(a);
            }
        }

        // Move the masses at the head of the prediction
        for mass in &mut self.masses {
            mass.move_seconds(seconds, self.positions_index);
        }
    }

    // initially simulate all the future positinos
    pub fn predict_positions(&mut self, simulation: &mut Simulation) {
        // All masses are there, calculate the simulation seconds by the maximal orbit time (as ui-time!)
        simulation.simulated_seconds_per_step = self.maximal_orbit_seconds()
            / SIMULATION_STEPS_PER_UI_TIME
            / simulation.ui_time_per_orbit;

        for _ in 1..PREDICT_COUNT {
            self.inc_position();
            self.drag_and_move(simulation.simulated_seconds_per_step);
        }
        self.inc_position(); // wrap to 0
    }

    pub fn inc_position(&mut self) {
        self.positions_index += 1;
        self.positions_index %= PREDICT_COUNT;
    }

    pub fn draw_position_0(&self, mass: &Mass) {
        mass.draw(0, self.predict_show as usize, self.predict_count);
    }

    pub fn draw_masses(&self) {
        for mass in &self.masses {
            mass.draw(
                self.positions_index,
                self.predict_show as usize,
                self.predict_count,
            );

            // this should be in mass:
            if mass.lagrange > 0 {
                let position_index =
                    (self.predict_show + self.positions_index as f64) % self.predict_count;
                let relativ_position =
                    self.orbit_relativ_position(mass.index, position_index as usize);
                // only l4?l5 yet
                let l4_angle: f64 = (60.0_f64).to_radians();
                let l4_position = relativ_position.rotate_angle(l4_angle); // + mass.position();
                draw_circle(l4_position, 22., RED);
            }
        }
    }
}
