//! Collection and instances of masses, like sun moon and stars
//! and mass related values, calculated of them

use crate::canvas::*;
use crate::mass::*;
use crate::vec_space::*;
use macroquad::prelude::RED;

// ------------------- MASS-DATA STRUCT/CLASS -------------------

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
    // Amound of visible prediction
    predict_count: f64,
    // Predict-index to show the mass
    predict_show: f64,
    follow: i32,
    rotation_a: usize, // why is this done in ship??? move to masses?
    rotation_b: usize,
}

impl Masses {
    pub fn new() -> Masses {
        if PREDICT_COUNT > 10000 {
            panic!("PREDICT_COUNT>10000");
        }

        Masses {
            masses: Vec::new(),
            positions_index: 0,
            maximal_orbit_seconds: 1.,
            maximal_orbit_radius: 1.,
            predict_count: PREDICT_COUNT as f64,
            predict_show: 0.,
            follow: -1, // not following
            rotation_a: 0,
            rotation_b: 0,
        }
    }

    pub fn positions_index(&self) -> usize {
        self.positions_index
    }

    pub fn predict_show(&self) -> f64 {
        self.predict_show
    }
    pub fn maximal_orbit_seconds(&self) -> f64 {
        self.maximal_orbit_seconds
    }

    pub fn set_predict_show_and_count(&mut self, count: f64, show: f64) {
        self.predict_count = count.clamp(1., PREDICT_COUNT as f64); // range, limit
        self.predict_show = show.clamp(0., self.predict_count as f64);
    }

    pub fn set_follow(&mut self, val: usize) {
        self.follow = val as i32;
    }

    pub fn set_rotation(&mut self, a: usize, b: usize) {
        self.rotation_a = a;
        self.rotation_b = b;
    }

    pub fn mul_predict_count(&mut self, fakt: f64) {
        self.predict_count = (self.predict_count * fakt).clamp(1., PREDICT_COUNT as f64);
        println!("predict_count: {}", self.predict_count);
    }

    pub fn add_predict_show(&mut self, val: f64) {
        self.predict_show += val;
        if self.predict_show >= self.predict_count - 1. {
            self.predict_show = self.predict_count - 1.;
        }
    }

    pub fn ___add_at_place(&mut self, mass_data: &MassData) -> usize {
        let index = self.masses.len();
        let (mass, _) = Mass::new(mass_data, None, index);
        println!("\nadd_at_place: index={}", index);
        println!("mass_data: {:?}", mass_data);
        println!("mass p/v: {:?} {:?}", mass.position(), mass.velocity());
        self.masses.push(mass);
        index
    }

    // DIE NEUEN SIND "BY"
    // add_at_center ???
    pub fn add_by_center(&mut self, properties: MassProperties) -> usize {
        self.add_by_vector(properties, MassVector::ZERO)
    }

    pub fn add_by_vector(&mut self, properties: MassProperties, vector: MassVector) -> usize {
        let index = self.masses.len();

        let (mass, _) = Mass::new2(properties, vector, MassOrbit::ZERO, None, index);

        println!("\nadd_by_vector: index={}", index);
        println!("properties: {:?}", properties);
        println!("mass p/v: {:?} {:?}", mass.position(), mass.velocity());
        self.masses.push(mass);
        index
    }

    pub fn add_by_orbit(
        &mut self,
        properties: MassProperties,
        orbit: MassOrbit,
        orbits_index: usize,
    ) -> usize {
        let index = self.masses.len();
        let orbits = &mut self.masses[orbits_index];

        let (mut mass, orbit_seconds) =
            Mass::new2(properties, MassVector::ZERO, orbit, Some(orbits), index);

        mass.orbits = orbits_index;
        self.maximal_orbit_radius = orbit.radius.abs().max(self.maximal_orbit_radius);
        self.maximal_orbit_seconds = self.maximal_orbit_seconds.max(orbit_seconds);
        println!("\nadd_by_orbit: index={} orbits={}", index, orbits_index);
        println!("properties: {:?}", properties);
        println!("mass p/v: {:?} {:?}", mass.position(), mass.velocity());
        println!("max orbit: {}", self.maximal_orbit_radius);
        self.masses.push(mass);
        index
    }

    pub fn set_in_orbit(&mut self, mass_data: &MassData, orbits_index: usize) -> Mass {
        let index = self.masses.len();
        let orbits = &mut self.masses[orbits_index];

        let (mut mass, orbit_seconds) = Mass::new(mass_data, Some(orbits), index);

        mass.orbits = orbits_index;
        self.maximal_orbit_radius = mass_data.orbit_radius.abs().max(self.maximal_orbit_radius);
        self.maximal_orbit_seconds = self.maximal_orbit_seconds.max(orbit_seconds);
        println!("\nset_in_orbit: index={} orbits={}", index, orbits_index);
        println!("mass_data: {:?}", mass_data);
        println!("mass p/v: {:?} {:?}", mass.position(), mass.velocity());
        println!("max orbit: {}", self.maximal_orbit_radius);
        mass
    }

    pub fn ___add_in_orbit_lagrange(
        &mut self,
        data: &MassData,
        orbits: usize,
        lagrange: i8,
    ) -> usize {
        let index = self.masses.len();
        let mut mass = self.set_in_orbit(data, orbits);
        mass.lagrange = lagrange;
        self.masses.push(mass);
        index
    }

    pub fn ___add_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        self.___add_in_orbit_lagrange(data, orbits, 0)
    }

    pub fn orbit_relativ_position(&self, mass_index: usize, predict_show: usize) -> VecSpace {
        let orbited_index = self.masses[mass_index].orbits;
        let orbited_position = self.masses[orbited_index].position();
        let mass_position = self.masses[mass_index].positions[predict_show];
        let relativ = mass_position - orbited_position;
        // println!("{:?} {:?}", mass_position, orbited_position);
        relativ
    }

    pub fn set_radius_to_draw(&self) {
        draw_set_maximal_orbit_radius(self.maximal_orbit_radius);
    }

    pub fn get_position(&self, index: usize) -> VecSpace {
        self.masses[index].positions[self.positions_index()]
    }

    pub fn get_draw_position(&self, mass_index: usize) -> VecSpace {
        let position_index = (self.positions_index + self.predict_show as usize) % PREDICT_COUNT;
        self.masses[mass_index].positions[position_index]
    }

    pub fn drag_at_position(&self, position: VecSpace, index: usize, interpolate: f64) -> VecSpace {
        let mut acceleration1 = VecSpace::ZERO;
        let mut acceleration2 = VecSpace::ZERO;
        let index2 = (index + 1) % PREDICT_COUNT;
        for mass in &self.masses {
            acceleration1 += mass.get_drag_for_that_position(position, index)
        }
        for mass in &self.masses {
            acceleration2 += mass.get_drag_for_that_position(position, index2)
        }
        acceleration1 * (1. - interpolate) + acceleration2 * interpolate
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

        // Move the masses at the draw-index of the prediction-positions
        for mass in &mut self.masses {
            mass.move_seconds(seconds, self.positions_index);
        }
    }

    // initially simulate the future positinos
    pub fn predict_positions(&mut self, simulated_seconds_per_step: f64) {
        for _ in 1..PREDICT_COUNT {
            self.inc_position();
            self.drag_and_move(simulated_seconds_per_step);
        }
        self.inc_position(); // wrap to 0
    }

    pub fn inc_position(&mut self) {
        self.positions_index += 1; // make next fn???
        self.positions_index %= PREDICT_COUNT;
    }

    pub fn draw_position_0(&self, mass: &Mass) {
        mass.draw(0, self.predict_show as usize, self.predict_count);
    }

    pub fn draw(&self) {
        if self.follow >= 0 {
            let position_of_followed_mass = self.get_draw_position(self.follow as usize);
            draw_set_view_xy(position_of_followed_mass * -1.);
        }

        let rotate = self.rotation_a != self.rotation_b;
        if rotate {
            let position_a = self.get_position(self.rotation_a);
            let position_b = self.get_position(self.rotation_b);
            let angle = (position_b - position_a).to_angle().to_degrees();
            //println!("angle: {}", angle);
            draw_set_view_angle(-angle);
        }

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
                // only L4?L5 yet
                let l4_angle: f64 = (60.0_f64).to_radians();
                let l4_position = relativ_position.rotate_angle(l4_angle); // + mass.position();
                draw_circle(l4_position, 22., RED);
            }
        }
    }
}
