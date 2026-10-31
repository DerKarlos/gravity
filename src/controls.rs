use crate::canvas::*;
use crate::mass::*;
use crate::ship::*;
use crate::simulation::*;
// for keyboard
use macroquad::prelude::*;

// ui-time gap to assume, the key was not pressed any more
const KEY_NOT_PRESSED_TIME: f64 = 0.33;

pub enum KeyEvent {
    NONE,
    BREAK,
    SceneID(i16),
}

pub struct Controls {
    x_view: AddSub,
    y_view: AddSub,
    z_view: AddSub,
    a_view: AddSub,
    speed: AddSub,
    predict_show: AddSub,
    burn_start: UpDown,
    burn_seconds: UpDown,
    burn_acceleration: UpDown,
}

impl Controls {
    pub fn new(ship: &Ship) -> Controls {
        let (burn_start, burn_seconds, burn_a) = ship.burn_values();
        Controls {
            x_view: AddSub::new("x_view", 50., 1., 1000.), // value, min, step, min
            y_view: AddSub::new("y_view", 50., 1., 1000.),
            z_view: AddSub::new("z_view", 1.0, 1., 1000.),
            a_view: AddSub::new("a_view", 50., 1., 1000.),
            speed: AddSub::new("speed", 1., 0.1, 10.),
            predict_show: AddSub::new("p_show", 10., 1., PREDICT_COUNT as f64),
            // predict_show: UpDown::new("p_show", Formular::Add, 0., 0., 10., 1.),
            burn_start: UpDown::new("b_start", Formular::Mul, burn_start, 0.0001, 1., 0.001),
            burn_seconds: UpDown::new("b_secs", Formular::Mul, burn_seconds, 0.0000, 1., 0.001), // min = 0 = off
            burn_acceleration: UpDown::new("power", Formular::Mul, burn_a, 0.0001, 1., 0.001),
        }
    }

    pub fn check_chars(&self, simulation: &mut Simulation, ship: &Ship) -> KeyEvent {
        if let Some(char) = get_char_pressed() {
            // println!("pressed char {:?}!", char);
            match char {
                '\u{1b}' => return KeyEvent::BREAK, // KeyCode::Escape
                '\r' => {
                    // KeyCode::Enter
                    simulation.toggle_run_mode();
                    //println!(  "planing_mode: {} {}",simulation.run_mode, simulation.simulated_world_seconds );
                }

                'ß' => {
                    let (burn_start, burn_seconds, burn_acceleration) = ship.burn_values();
                    println!(
                        "burn start at {}s for {}s, a={}",
                        burn_start, burn_seconds, burn_acceleration
                    );
                }

                '0'..='9' | 'r' => {
                    let id = if char == 'r' {
                        simulation.scene()
                    } else {
                        char as i16 - 48
                    };
                    println!("scene = {}", id);
                    return KeyEvent::SceneID(id);
                }
                _ => (), // println!("Char not used: {:?}!", char),  //  (),
            }
        }
        KeyEvent::NONE
    }

    pub fn check_keys(&mut self, simulation: &mut Simulation, ship: &mut Ship, delta_time: f64) {
        let (_, _, burn_acceleration) = ship.burn_values();
        if is_key_down(KeyCode::Space) {
            ship.accelerate_ahead(burn_acceleration * delta_time);
        }
        if is_key_down(KeyCode::Backspace) {
            ship.accelerate_ahead(-burn_acceleration * delta_time)
        }

        ship.set_start_seconds(
            self.burn_start
                .up_down(KeyCode::Right, KeyCode::Left, delta_time), // ui-time
        );

        ship.set_burn_seconds(
            self.burn_seconds
                .up_down(KeyCode::Up, KeyCode::Down, delta_time),
        );

        ship.set_burn_acceleration(self.burn_acceleration.up_down(
            KeyCode::RightBracket,
            KeyCode::Apostrophe,
            delta_time,
        ));

        if is_key_down(KeyCode::O) {
            simulation.masses.mul_predict_count(1.003);
        }
        if is_key_down(KeyCode::L) {
            simulation.masses.mul_predict_count(0.996);
        }

        //  .up_down(KeyCode::I, KeyCode::K, delta_time);
        //masses.set_predict_show(actual);
        simulation
            .masses
            .add_predict_show(
                self.predict_show
                    .add_sub(KeyCode::I, KeyCode::K, delta_time),
            );

        draw_add_view(
            self.x_view.add_sub(KeyCode::A, KeyCode::D, delta_time),
            self.y_view.add_sub(KeyCode::W, KeyCode::S, delta_time),
            self.z_view.add_sub(KeyCode::E, KeyCode::Q, delta_time),
            self.a_view.add_sub(KeyCode::C, KeyCode::Z, delta_time), // Y on german keybpards! Do we need an Y or Z???
        );
        simulation.add_iterations(self.speed.add_sub(KeyCode::T, KeyCode::G, delta_time));
    }
}

#[derive(Clone, Copy)]
enum Formular {
    _Add,
    Mul,
}

struct UpDown {
    name: &'static str,
    formular: Formular,
    last_time: f64,
    actual: f64,
    actual_min: f64,
    step: f64,
    step_min: f64,
    last_was_up: bool,
}

impl UpDown {
    pub fn new(
        name: &'static str,
        formular: Formular,
        actual: f64,
        actual_min: f64,
        step: f64,
        step_min: f64,
    ) -> UpDown {
        UpDown {
            name,
            formular,
            last_time: 0.,
            actual,
            actual_min,
            step,
            step_min,
            last_was_up: true,
        }
    }

    fn step_down(&mut self) {
        self.step /= 2.;
        if self.step < self.step_min {
            self.step = self.step_min;
        }
        println!("{}: step_down {}", self.name, self.step);
    }

    pub fn up_down(&mut self, key_up: KeyCode, key_down: KeyCode, delta_time: f64) -> f64 {
        let mut set = self.actual;
        if is_key_down(key_up) {
            set = self.up(delta_time);
            println!("{} up: {} / {}", self.name, set, self.step);
        }
        if is_key_down(key_down) {
            set = self.down(delta_time);
            println!("{} down: {} / {}", self.name, set, self.step);
        }
        set
    }

    fn up(&mut self, delta_time: f64) -> f64 {
        let time = get_time();
        if time - self.last_time > KEY_NOT_PRESSED_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends
            self.step *= 1. + 1. * delta_time;
        } else {
            self.last_was_up = true;
            self.step_down();
        }

        // println!("up {} - {}", self.actual, self.step);
        // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends

        match self.formular {
            Formular::Mul => self.actual *= 1. + 1. * delta_time * self.step,
            Formular::_Add => self.actual += delta_time * self.step,
        }

        self.actual
    }
    fn down(&mut self, delta_time: f64) -> f64 {
        let last_step = self.step;
        let time = get_time();
        if time - self.last_time > KEY_NOT_PRESSED_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            self.last_was_up = false;
            self.step_down();
        } else {
            self.step *= 1. + 1. * delta_time;
        }

        // println!("dn {} - {}", self.actual, self.step);
        // self.actual *= 1. - 1. * delta_time * self.step;
        match self.formular {
            Formular::Mul => self.actual *= 1. - 1. * delta_time * self.step,
            Formular::_Add => self.actual -= delta_time * self.step,
        }

        if self.actual < self.actual_min {
            self.actual = self.actual_min;
            self.step = last_step;
        }

        self.actual
    }
}

struct AddSub {
    name: &'static str,
    last_time: f64,
    step: f64,
    min: f64,
    max: f64,
    last_was_up: bool,
}

impl AddSub {
    pub fn new(name: &'static str, step: f64, min: f64, max: f64) -> AddSub {
        AddSub {
            name,
            last_time: 0.,
            step,
            min,
            max,
            last_was_up: true,
        }
    }

    fn step_down(&mut self) {
        self.step /= 2.;
        self.step = self.step.clamp(self.min, self.max);
        println!("{}: step_down {}", self.name, self.step);
    }

    // ramps the step, the values are changed from
    // and returns its value for the given delta time to be added to the value
    pub fn add_sub(&mut self, key_up: KeyCode, key_down: KeyCode, delta_time: f64) -> f64 {
        let mut add = 0.;
        if is_key_down(key_up) {
            add = self.add(delta_time) * delta_time;
            println!("{} add: {} / {}", self.name, add, self.step);
        }
        if is_key_down(key_down) {
            add = self.sub(delta_time) * delta_time;
            println!("{} sub: {} / {}", self.name, add, self.step);
        }
        add
    }

    // increases the step, the values are changed from
    fn add(&mut self, delta_time: f64) -> f64 {
        let time = get_time();
        if time - self.last_time > KEY_NOT_PRESSED_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends
            self.step *= 1. + 1. * delta_time;
        } else {
            self.last_was_up = true;
            self.step_down();
        }

        // println!("up {} - {}", self.actual, self.step);
        self.step
    }
    // decreases the step, the values are changed from
    fn sub(&mut self, delta_time: f64) -> f64 {
        let time = get_time();
        if time - self.last_time > KEY_NOT_PRESSED_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            self.last_was_up = false;
            self.step_down();
        } else {
            self.step *= 1. + 1. * delta_time;
        }

        // println!("dn {} - {}", self.actual, self.step);
        -self.step
    }
}
