//! Collection and instances of masses, like sun moon and stars
//! and mass related values, calculated of them

use crate::canvas::*;
use crate::vec_space::*;
use macroquad::prelude::{Color, GREEN};

//////////////////// Parameter ////////////////////

pub const PREDICT_COUNT: usize = 1000; // ttt

pub const GRAVITY_CONSTANT_OF_EARTH: f64 = 6.67384e-11; // m^3/(kg*s^2)

// ------------------- SI UNIT VALUE KONVERT OPTIONS  -------------------

// distances

/// Kilometer to SI meter
pub const fn km(km: f64) -> f64 {
    km * 1000.
}

/// Astronomical units to SI meter
pub const fn au(au: f64) -> f64 {
    au * 149_597_870_700.0
}

/// Masses in kilogram (wheight), just to show the SI unit
pub const fn kg(kg: f64) -> f64 {
    kg
}

/// Earth masses to SI kilogram   // earth_wheight ??? mass = mass ??? himmelskörper = celestial body
pub const fn mass_earth(earth: f64) -> f64 {
    earth * 5.974e24
}

/// Sol/Sun masses to SI kilogram
pub const fn mass_sol(sol: f64) -> f64 {
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

// ------------------- MASS-DATA STRUCT/CLASS -------------------

#[derive(Debug, Default, Clone, Copy)]
pub struct MassProperties<'a> {
    name: &'a str,
    mass: f64,
    diameter: f64,
    color: Color,
}

impl<'a> MassProperties<'a> {
    pub const fn new(name: &str, mass: f64, diameter: f64, color: Color) -> MassProperties<'_> {
        MassProperties {
            name,
            mass, // earth_wheight ??? mass = mass ??? himmelskörper = celestial body
            diameter,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MassVector {
    position: VecSpace,
    velocity: VecSpace,
}
impl MassVector {
    pub const fn new(position: VecSpace, velocity: VecSpace) -> MassVector {
        MassVector { position, velocity }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MassOrbit {
    pub radius: f64,
    excentricity: f64,
}
impl MassOrbit {
    pub const fn new(radius: f64, excentricity: f64) -> MassOrbit {
        MassOrbit {
            radius,
            excentricity,
        }
    }
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
    pub index: usize,
    mass: f64,
    diameter: f64,
    pub orbits: usize,
    pub lagrange: i8,
    color: Color,
    acceleration: VecSpace,
    velocity: VecSpace,
    position: VecSpace,
    pub positions: [VecSpace; PREDICT_COUNT], // rename to trajectory ???
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

    pub fn new_at_vector(
        properties: &MassProperties,
        vector: &MassVector,
        index: usize,
    ) -> (Mass, f64) {
        let mut mass = Mass {
            _name: properties.name.to_string(),
            index,
            color: properties.color,
            diameter: properties.diameter,
            orbits: 0,
            lagrange: 0,
            mass: properties.mass,
            position: vector.position,
            velocity: vector.velocity,
            acceleration: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        };

        let orbit_seconds: f64 = 0.;
        mass.positions[0] = mass.position;
        (mass, orbit_seconds)
    }

    pub fn new_in_orbit(
        properties: &MassProperties,
        orbit: &MassOrbit,
        orbited_mass: &mut Mass,
        index: usize,
    ) -> (Mass, f64) {
        let mut mass = Mass {
            _name: properties.name.to_string(),
            index,
            color: properties.color,
            diameter: properties.diameter,
            orbits: 0,
            lagrange: 0,
            mass: properties.mass,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            acceleration: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        };

        mass.orbits = orbited_mass.index;
        mass.position += VecSpace::new(orbit.radius, 0.); // + orbited_mass.position; done by set_v_orbit
        let orbit_seconds = Self::set_v_orbit(&mut mass, orbited_mass, orbit.excentricity);

        mass.positions[0] = mass.position;
        (mass, orbit_seconds)
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

    // pub struct MassData<'a> {
    //     name: &'a str,

    pub fn _ne2(
        name: &str,
        color: Color,
        diameter: f64,
        mass: f64,
        orbit_radius: f64,
        excentricity: f64,
        orbits: Option<&mut Mass>,
        index: usize,
    ) -> (Mass, f64) {
        let mut mass = Mass {
            _name: name.to_string(),
            index,
            color,
            diameter,
            orbits: 0,
            lagrange: 0,
            mass,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            acceleration: VecSpace::ZERO,
            positions: [VecSpace::ZERO; PREDICT_COUNT],
        };

        let mut orbit_seconds: f64 = 0.;
        if let Some(orbited_mass) = orbits {
            mass.orbits = orbited_mass.index;
            mass.position += VecSpace::new(orbit_radius, 0.); // + orbited_mass.position; done by set_v_orbit
            orbit_seconds = Self::set_v_orbit(&mut mass, orbited_mass, excentricity);
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

    pub fn delta_step(&self, predict_show: usize) -> VecSpace {
        self.positions[predict_show] - self.positions[(predict_show + 1) % PREDICT_COUNT]
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

    pub fn drag(&self, other: &mut Mass) {
        let mut distance_vector = self.position - other.position;
        let distance = distance_vector.length();
        distance_vector.normalize();

        // F = force (N) : m = mass (kg) / r² = distance² (m²) * G = 6.67430 × 10⁻¹¹ m³/(kg·s²)
        let acceleration = self.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
        let acceleration_vector = distance_vector * acceleration;

        other.acceleration += acceleration_vector;
    }

    pub fn get_drag_for_that_position(
        &self,
        position: VecSpace,
        position_index: usize,
    ) -> VecSpace {
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
            // The drawed size is not proportional to the real size to avoid big differences
            self.diameter.sqrt().sqrt(),
            self.color,
        );
        //println!("x/y {}/{}", screen_pos.x() as f32, screen_pos.y() as f32);

        //draw_rectangles(
        //    &self.positions,
        //    self.color,
        //    positions_index,
        //    predict_count as usize,
        //);

        for index in 0..predict_count as usize {
            let index = (index + positions_index) % PREDICT_COUNT;
            let position = self.positions[index];
            draw_rectangle(position, self.color);
        }
    }
}
