use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 680; // ??? calculate frame
const WINDOW_CENTER: Vec2 = Vec2 {
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
    pub diameter_km: f64,
    pub mass: f64,
    pub radius: f64,
    pub excentricity: f64,
}

#[derive(Debug, Clone)]
pub struct Mass {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    color: Color,
    _name: String,
    mass: f64,
    diameter: f64,
}

impl Mass {
    // "Static" constants

    pub fn new(data: &MassData, orbits: Option<&mut Mass>) -> Mass {
        let position = Vec2::new(0.0, data.radius * M_AE);
        let velocity = Vec2::null();
        let acceleration = Vec2::null();

        let mut mass = Mass {
            _name: data.name.to_string(),
            color: data.color,
            diameter: data.diameter_km * 1000.0, // km → m
            mass: data.mass,
            position,
            velocity,
            acceleration,
        };

        if orbits.is_none() {
            return mass;
        }

        let mut parent = orbits.unwrap();
        // let rel = position.sub(parent.position);
        // let r = rel.length();
        // if r == 0.0 {
        //     return mass;
        // }

        Self::mass_v_orbit(&mut mass, &mut parent, data.excentricity);

        // let v_mag = Self::v_orbit(parent.mass, r);
        // // perpendicular direction to radius for circular orbit
        // let tangential = Vec2::new(-rel.y / r, rel.x / r);
        // let orbit_velocity = tangential.mul(v_mag);
        // mass.velocity = velocity.add(orbit_velocity).add(parent.velocity);

        return mass;
    }

    /// Computes orbital velocity for a circular orbit
    /// around a body with `central_mass` at distance `radius_m` (in meters)

    fn mass_v_orbit(mass: &mut Mass, other: &mut Mass, excentriticy: f64) {
        let signum = if mass.position.y > 0.0 { 1.0 } else { -1.0 };
        mass.position.add(other.position);
        let radius = other.position.sub(mass.position).length();

        let both_masses = mass.mass + other.mass;
        let velocity =
            (GRAVITY_CONSTANT_OF_EARTH * both_masses / radius).sqrt() * (1. - excentriticy);
        mass.velocity.x += velocity / both_masses * other.mass * signum;
        other.velocity.x += -velocity / both_masses * mass.mass * signum;

        /* * /
        with (m) {
            var si  = py>0 ? +1 : -1;
              // Positon und Geschwindigkeit der neuen Masse ist relativ zur umkreisten Masse
                  px += o.px;  vx += o.vx;
                  py += o.py;  vy += o.vy;
            var r   = Phytagoras(  o.px-px
                                          ,  o.py-py);           // Radius/Abstand, Masse zur anderen Masse
              // console.log(o.px,px,o.py,py,r,'x,x,y,y,r')
                  // Für Kreisbahn notwendige Geschwindigkeit errechnen
                  r   = r * mAE;
            var mpm = mass + o.mass;
            var v   = Math.sqrt(2*g*mpm/r);  // wikipedia: Fluchtgeschwindigkeit - Zweite kosmische Geschwindigkeit,
                  // Geschwindigkeit entsprechend der Massen verteilen
                  v   = v / mAE;  // Aus m/s^2 werden AE/s^2
                      vx += +v / mpm * o.mass * -si; // Eigene v für Orbit
                  o.vx += -v / mpm *   mass * -si; // Andere v für Orbit
              // console.log(m,o,r,mpm,v,vx,o.vx,'(m,o,r,mpm,v,vx,o.vx)')
          }
        }
        / * */
    }

    fn _v_orbit(central_mass: f64, radius_m: f64) -> f64 {
        (GRAVITY_CONSTANT_OF_EARTH * central_mass / radius_m).sqrt()
    }

    pub fn dragged_by(&mut self, other: &Mass) {
        if (self as *const _) == (other as *const _) || other.mass == 0.0 {
            return; // don’t drag self or zero-mass objects
        }

        let mut distance_vector = other.position.sub(self.position);
        let distance = distance_vector.length();

        if distance < MAX_GRAVITY_DISTANCE * M_AE {
            distance_vector.normalize();

            let acceleration = other.mass / (distance * distance) * GRAVITY_CONSTANT_OF_EARTH;
            let acceleration_vector = distance_vector.mul(acceleration);

            self.acceleration = self.acceleration.add(acceleration_vector);
        }
    }

    pub fn frame_move(&mut self, dt: f64) {
        self.velocity = self.velocity.add(self.acceleration.mul(dt));
        self.position = self.position.add(self.velocity.mul(dt));
        self.acceleration.set_zero();
    }

    pub fn draw(&self) {
        // sqrt(sqrt()) scaling like Kotlin code
        let mut size = ((self.diameter / M_AE).sqrt().sqrt() / 2.0 * DRAW_FACT) as i32;
        size = size.clamp(DRAW_MIN, DRAW_MAX);

        let screen_pos = scale(&self.position);
        draw_circle(
            screen_pos.x as f32,
            screen_pos.y as f32,
            size as f32,
            self.color,
        );
    }
}

fn scale(position: &Vec2) -> Vec2 {
    let z_view = 1.2;
    position
        .mul(PIXEL as f64 / z_view / M_AE)
        .add(WINDOW_CENTER)
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

    pub fn frame(&mut self, seconds_per_frame: f64) {
        // each mass drags each other mass, except itselfes
        for mass_index in 0..self.masses.len() {
            // todo: don't clone
            let mass = self.masses[mass_index].clone();
            for dragged in &mut self.masses {
                // todo: no string compare
                if mass._name != dragged._name {
                    dragged.dragged_by(&mass);
                }
            }
        }

        for mass in &mut self.masses {
            mass.frame_move(seconds_per_frame);
            mass.draw();
        }
    }
}

// ------------------- VEC2 STRUCT/CLASS -------------------
// use own file???
#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn null() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn add(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    fn sub(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    fn mul(&self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }

    fn normalize(&mut self) {
        let len = self.length();
        // len >= epsilon
        if len != 0.0 {
            self.x /= len;
            self.y /= len;
        }
    }

    fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
