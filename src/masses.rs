use crate::canvas::*;
use crate::simulation::*;
use crate::vec_space::*;
use macroquad::prelude::*;

/// Collection and instances of masses, like sun moon and stars
/// and mass related values, calculated of them

///// Parameter /////

pub const PREDICT_COUNT: usize = 1000;

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)

//b const MAX_GRAVITY_DISTANCE: f64 = 1e38; // [AE]

// ------------------- Globals   -------------------

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
        let mut ret = self.clone();
        ret.orbit_radius *= fakt;
        ret
    }
}

// =================== MASS STRUCT/CLASS ===================

// avoid pub???
#[derive(Debug, Clone)]
pub struct Mass {
    _name: String, // why not &str ???
    mass: f64,
    diameter: f64,
    orbit_time: f64,
    color: Color,
    acceleration: VecSpace,
    pub velocity: VecSpace,
    pub position: VecSpace,
    positions: [VecSpace; PREDICT_COUNT],
}

impl Mass {
    pub fn _zero() -> Mass {
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

    // pub for new ship
    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        // ignore radius if mass is not in orbit
        let position = if orbits.is_some() {
            VecSpace::new(data.orbit_radius, 0.0)
        } else {
            VecSpace::ZERO
        };

        let mut mass = Mass {
            _name: data.name.to_string(),
            color: data.color,
            diameter: data.diameter,
            orbit_time: 0.,
            mass: data.mass,
            position,
            velocity: VecSpace::ZERO,
            acceleration: VecSpace::ZERO,
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

    pub fn ship_accelerate_ahead(&mut self, acceleration: f64) {
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

    fn drag_position(&self, position: VecSpace, position_index: usize) -> VecSpace {
        let mut distance_vector = self.positions[position_index] - position;
        let distance = distance_vector.length();
        distance_vector.normalize();

        // F = force (N) : m = mass (kg) / r² = distance² (m²) * G = 6.67430 × 10⁻¹¹ m³/(kg·s²)
        let acceleration = self.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
        let acceleration_vector = distance_vector * acceleration;

        acceleration_vector
    }

    pub fn move_seconds(&mut self, seconds: f64, positions_index: usize) {
        self.velocity += self.acceleration * seconds;
        self.position += self.velocity * seconds;
        self.positions[positions_index] = self.position;
        self.acceleration.set_zero();
    }

    // do it by thread_local ?
    pub fn draw(&self, canvas: &Canvas, positions_index: usize) {
        canvas.draw_circle(
            &self.positions[positions_index],
            // visible size not real and less proportional to avoid big differences
            self.diameter.sqrt().sqrt(),
            self.color,
        );
        //println!("x/y {}/{}", screen_pos.x() as f32, screen_pos.y() as f32);

        for position in &self.positions {
            canvas.draw_rectangle(position, self.color);
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
    maximal_orbit_time: f64,
    // Calculated by the masses. Also needed and copied to the canvas.
    maximal_orbit_radius: f64,
}

impl Masses {
    pub fn new() -> Masses {
        Masses {
            masses: Vec::new(),
            positions_index: 0,
            maximal_orbit_time: 1.,
            maximal_orbit_radius: 1.,
        }
    }

    pub fn positions_index(&self) -> usize {
        self.positions_index
    }

    pub fn maximal_orbit_time(&self) -> f64 {
        self.maximal_orbit_time
    }

    pub fn add_at_place(&mut self, data: &MassData) -> usize {
        let mass = Mass::new(data, None);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        let orbits = &mut self.masses[orbits];
        self.maximal_orbit_radius = data.orbit_radius.max(self.maximal_orbit_radius);
        //println!("max orbit: {}", self.maximal_orbit);
        let mass = Mass::new(data, Some(orbits));
        self.maximal_orbit_time = self.maximal_orbit_time.max(mass.orbit_time);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn set_radius(&self, canvas: &mut Canvas) {
        canvas.set_maximal_orbit_radius(self.maximal_orbit_radius);
    }

    pub fn get_from_index(&mut self, index: usize) -> &mut Mass {
        &mut self.masses[index]
    }

    pub fn drag_at_position(&self, position: VecSpace, index: usize) -> VecSpace {
        let mut acceleration = VecSpace::ZERO;
        for mass in &self.masses {
            acceleration += mass.drag_position(position, index)
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
        // All masses are there, calculate the simulation time by the maximal orbit time
        simulation.simulated_seconds_per_step =
            self.maximal_orbit_time() / SIMULATION_STEPS_PER_SECOND / simulation.seconds_per_orbit;

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

    pub fn get_position(&self) -> usize {
        self.positions_index
    }

    pub fn draw(&self, canvas: &Canvas) {
        for mass in &self.masses {
            mass.draw(canvas, self.positions_index);
        }
    }
}
